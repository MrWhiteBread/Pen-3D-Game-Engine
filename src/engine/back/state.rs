mod render_fn;
mod compute_fr;
mod mipmap;

use std::iter;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use arc_swap::access::{DynAccess, DynGuard};
use arc_swap::ArcSwap;
use atomic_float::{AtomicF32, AtomicF64};
use wgpu::{Backends, Buffer, BufferAddress, BufferDescriptor, BufferUsages, CompositeAlphaMode, Device, DeviceDescriptor, ExperimentalFeatures, Extent3d, Features, Instance, InstanceDescriptor, Limits, MemoryHints, PowerPreference, PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureFormat, TextureUsages, Trace};
use wgpu::wgt::TextureViewDescriptor;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event::WindowEvent::KeyboardInput;
use winit::window::Window;
use crate::engine::back::buffers::Buffers;
use crate::engine::back::screen_textures::ScreenTextures;
use crate::engine::back::key_handler::InputHandler;
use crate::engine::back::pipelines::Pipelines;
use crate::engine::back::settings::Settings;
use crate::engine::back::staging_buffers::StagingBuffers;
use crate::engine::back::types::camera::CameraUniform;
use crate::engine::back::types::globals::Globals;
use crate::engine::back::types::light::LightType;
use crate::engine::back::types::object::Object;
use crate::engine::back::types::vertex::Vertex;

pub struct State<'a> {
    pub settings: &'a mut Settings,
    buffers: Buffers,
    staging_buffers: Arc<ArcSwap<StagingBuffers>>,
    pipelines: Pipelines,
    
    instance: Instance,
    surface: Surface<'a>,
    device: Device,
    queue: Arc<Queue>,
    config: SurfaceConfiguration,

    black_vertices_buffer: Buffer,
    black_indices_buffer: Buffer,

    screen_textures: ScreenTextures,
    
    pub key_handler: Arc<ArcSwap<InputHandler>>,
    pub cursor_x: Arc<AtomicF32>,
    pub cursor_y: Arc<AtomicF32>,
    pub cursor_locked: Arc<AtomicBool>,
    pub window_bounds: Arc<ArcSwap<([i32; 2], [u32; 2], bool, bool)>>,
    
    pub globals_updated: Arc<AtomicBool>,
    pub ticks_updated: Arc<AtomicBool>,
    pub del_timer: Arc<AtomicF64>,
}

impl<'a> State<'a> {
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture().expect("Couldn't get output");
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let staging_buffer: DynGuard<Arc<StagingBuffers>> = self.staging_buffers.load();

        self.update_buffers(&mut encoder, &staging_buffer);
        self.compute_pass(&mut encoder, &staging_buffer);

        let object_count = staging_buffer.object_count.clone();
        drop(staging_buffer);

        if self.settings.deferred_rendering {
            self.deferred_rendering(&mut encoder, &view, &object_count);
        } else {
            self.forward_rendering(&mut encoder, &view, &object_count);
        }

        //self.mipmap_rendering(&mut encoder, 5, &self.screen_textures.light_rays_texture);
        self.light_rays_rendering(&mut encoder, &view);

        self.queue.submit(iter::once(encoder.finish()));
        output.present();
        let _ = self.device.poll(wgpu::PollType::wait_indefinitely());

        Ok(())
    }

    fn update_buffers(&self, encoder: &mut wgpu::CommandEncoder, staging_buffers: &Arc<StagingBuffers>) {
        let del_time = (self.del_timer.load(Ordering::Relaxed) / self.settings.time_per_update) as f32;
        let global = Globals {
            t: del_time,
            scene_light: staging_buffers.scene_light,
            old_scene_light: staging_buffers.old_scene_light,
            lerp_scene_light: 0.0,

            screen_size: [self.settings.size.width as f32, self.settings.size.height as f32],

            padding: [0.0; 2],
        };

        self.queue.write_buffer(
            &self.buffers.globals,
            0,
            bytemuck::bytes_of(&global),
        );

        self.queue.write_buffer(
            &self.buffers.count_light_buffer,
            0,
            bytemuck::bytes_of(&0),
        );

        self.queue.write_buffer(
            &self.buffers.count_buffer,
            0,
            bytemuck::bytes_of(&0),
        );

        let camera = CameraUniform::new_from_struct(&staging_buffers.camera_structure[0].lerp(&staging_buffers.camera_structure[1], &del_time));

        self.queue.write_buffer(
            &self.buffers.camera_buffer,
            0,
            bytemuck::bytes_of(&camera),
        );

        if self.ticks_updated.load(Ordering::Relaxed) {
            self.ticks_updated.store(false, Ordering::Relaxed);

            self.queue.write_buffer(
                &self.buffers.object_count_buffer,
                0,
                bytemuck::bytes_of(&staging_buffers.object_count),
            );

            self.queue.write_buffer(
                &self.buffers.light_count_buffer,
                0,
                bytemuck::bytes_of(&staging_buffers.light_count),
            );

            encoder.copy_buffer_to_buffer(
                &staging_buffers.vertices_staging,
                0,
                &self.buffers.vertices_buffer,
                staging_buffers.vertices_dest_offset,
                (size_of::<Vertex>() * staging_buffers.vertices_count as usize) as BufferAddress,
            );

            encoder.copy_buffer_to_buffer(
                &staging_buffers.indices_staging,
                0,
                &self.buffers.indices_buffer,
                staging_buffers.indices_dest_offset,
                (size_of::<u32>() * staging_buffers.indices_count as usize) as BufferAddress,
            );

            encoder.copy_buffer_to_buffer(
                &staging_buffers.object_staging,
                0,
                &self.buffers.object_buffer,
                staging_buffers.object_dest_offset,
                (size_of::<Object>() * staging_buffers.object_count as usize) as BufferAddress,
            );

            encoder.copy_buffer_to_buffer(
                &staging_buffers.light_staging,
                0,
                &self.buffers.light_buffer,
                staging_buffers.light_dest_offset,
                (size_of::<LightType>() * staging_buffers.light_count as usize) as BufferAddress,
            );
        }
    }


    pub fn input(&mut self, event: &WindowEvent) -> bool {
        let guard = ArcSwap::<InputHandler>::load(&self.key_handler);
        let mut key_handler = (**guard).clone();
        let mut changed = false;

        match event {
            WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = *state == ElementState::Pressed;
                key_handler.update_mouse(*button, is_pressed);
                changed = true;
            }

            KeyboardInput {
                event: key_event, ..
            } => {
                let is_pressed = key_event.state == ElementState::Pressed;
                let key = key_event.logical_key.clone();
                key_handler.update_keys(key, is_pressed);
                changed = true;
            }

            _ => {},
        }

        if changed {
            self.key_handler.store(Arc::new(key_handler));
            true
        } else {
            false
        }
    }
    
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if self.settings.old_size != new_size && new_size.width > 0 && new_size.height > 0 {
            self.instance.poll_all(true);
            self.settings.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.remake_globals_textures();
        }
    }

    fn remake_globals_textures(&mut self) {
        let size = Extent3d {
            width: self.config.width,
            height: self.config.height,
            depth_or_array_layers: 1,
        };

        self.screen_textures = ScreenTextures::new(&self.device, size.width, size.height);

        self.pipelines.remake_deferred_f_bind_group(&self.device, &self.screen_textures);
        self.pipelines.remake_forward_screen_bind_group(&self.device, &self.screen_textures);
        self.pipelines.remake_pp_forward_screen_bind_group(&self.device, &self.screen_textures);
        self.pipelines.remake_light_rays_bind_group(&self.device, &self.buffers, &self.screen_textures);
    }
    
    pub async fn new(window: &'a Window, settings: &'a mut Settings) -> (Self, Arc<ArcSwap<StagingBuffers>>) {
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::DX12,
            ..Default::default()
        });
        
        let surface = instance.create_surface(window).expect("Can't create surface");
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.expect("Can't get adapter");
        
        let device_descriptor = DeviceDescriptor {
            label: Some("My Device"),
            required_features: Features::INDIRECT_FIRST_INSTANCE | Features::MULTI_DRAW_INDIRECT_COUNT | Features::TEXTURE_BINDING_ARRAY | Features::TEXTURE_FORMAT_16BIT_NORM,
            required_limits: Limits::default(),
            experimental_features: ExperimentalFeatures::default(),
            memory_hints: MemoryHints::Performance,
            trace: Trace::default(),
        };
        
        let (device, queue) = adapter.request_device(&device_descriptor).await.expect("Can't create device");
        let queue = Arc::new(queue);
        
        let surface_caps = surface.get_capabilities(&adapter);
        let format = TextureFormat::Rgba8Unorm;
        let size = window.inner_size();
        
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Immediate,
            desired_maximum_frame_latency: 0,
            alpha_mode: surface_caps
                .alpha_modes
                .iter()
                .copied()
                .find(|m| *m != CompositeAlphaMode::Opaque)
                .unwrap_or(CompositeAlphaMode::Opaque),
            view_formats: vec![],
        };
        settings.size = size;
        settings.old_size = PhysicalSize::new(0, 0);
        surface.configure(&device, &config);

        let screen_textures = ScreenTextures::new(&device, config.width, config.height);

        let black_vertices: [f32; 8] = [
            -1.0, 1.0,
            1.0, 1.0,
            1.0, -1.0,
            -1.0, -1.0,
        ];

        let black_indices = [0, 3, 2, 2, 1, 0];

        let black_vertices_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("black_vertices_buffer"),
            size: (size_of::<f32>() * 8) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let black_indices_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("black_indices_buffer"),
            size: (size_of::<u32>() * 6) as u64,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&black_vertices_buffer, 0, bytemuck::cast_slice(&black_vertices));
        queue.write_buffer(&black_indices_buffer, 0, bytemuck::cast_slice(&black_indices));

        let info = adapter.get_info();
        println!("Using GPU: {} ({:?})", info.name, info.device_type);
        
        let (buffers, staging_buffers) = Buffers::new(&device, &info, queue.clone());
        let pipelines = Pipelines::new(&device, &buffers, &config, &screen_textures);
        let staging_buffers = Arc::new(ArcSwap::new(Arc::new(staging_buffers)));

        (
            Self {
                settings,
                buffers,
                staging_buffers: staging_buffers.clone(),
                pipelines,
                
                instance,
                surface,
                device,
                queue,
                config,

                black_vertices_buffer,
                black_indices_buffer,
                
                screen_textures,
                key_handler: Arc::new(ArcSwap::from_pointee(InputHandler::new())),
                cursor_x: Arc::new(AtomicF32::new(0.0)),
                cursor_y: Arc::new(AtomicF32::new(0.0)),
                cursor_locked: Arc::new(AtomicBool::new(false)),
                window_bounds: Arc::new(ArcSwap::new(Arc::new(([window.outer_position().expect("idk").x, window.outer_position().expect("idk").y], [window.outer_size().width, window.outer_size().height], false, false)))),
                
                globals_updated: Arc::new(AtomicBool::new(false)),
                ticks_updated: Arc::new(AtomicBool::new(false)),
                del_timer: Arc::new(AtomicF64::new(0.0)),
            },
            
            staging_buffers,
        )
    }
}