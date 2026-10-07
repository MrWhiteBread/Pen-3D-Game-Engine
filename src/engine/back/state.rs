mod update_buffers;
mod render;

use std::iter;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool};
use arc_swap::access::{DynAccess, DynGuard};
use arc_swap::ArcSwap;
use atomic_float::{AtomicF32, AtomicF64};
use wgpu::{Backends, Buffer, BufferDescriptor, BufferUsages, CompositeAlphaMode, CurrentSurfaceTexture, Device, DeviceDescriptor, ExperimentalFeatures, Extent3d, Features, Instance, InstanceDescriptor, Limits, MemoryHints, PowerPreference, PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceColorSpace, SurfaceConfiguration, TextureFormat, TextureUsages, Trace};
use wgpu::wgt::TextureViewDescriptor;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event::WindowEvent::KeyboardInput;
use winit::window::Window;
use crate::engine::back::buffers::Buffers;
use crate::engine::back::global_textures::GlobalTextures;
use crate::engine::back::screen_textures::ScreenTextures;
use crate::engine::back::key_handler::InputHandler;
use crate::engine::back::pipelines::Pipelines;
use crate::engine::back::settings::Settings;
use crate::engine::back::staging_buffers::StagingBuffers;

pub struct State<'a> {
    pub settings: Settings,
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

    global_textures: GlobalTextures,
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
    pub fn render(&mut self) {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });


        let staging_buffer: DynGuard<Arc<StagingBuffers>> = self.staging_buffers.load();

        self.update_buffers(&mut encoder, &staging_buffer);
        self.compute_pass(&mut encoder, &staging_buffer);

        let object_count = staging_buffer.object_count.clone();
        drop(staging_buffer);


        let output = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture) => texture,

            CurrentSurfaceTexture::Suboptimal(texture) => texture,

            CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Lost =>  {
                self.surface.configure(&self.device, &self.config);
                return;
            }

            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded | CurrentSurfaceTexture::Validation => {
                return;
            }
        };

        let view = output.texture.create_view(&TextureViewDescriptor::default());

        if self.settings.deferred_rendering {
            self.deferred_rendering(&mut encoder, &view, &object_count);
        } else {
            self.forward_rendering(&mut encoder, &view, &object_count);
        }

        //self.mipmap_rendering(&mut encoder, 5, &self.screen_textures.lens_flare_texture);

        self.queue.submit(iter::once(encoder.finish()));
        self.queue.present(output);

        let _ = self.device.poll(wgpu::PollType::wait_indefinitely());
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

                let key = key_event.physical_key.clone();
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
    }
    
    pub async fn new(window: Arc<Window>, mut settings: Settings) -> (Self, Arc<ArcSwap<StagingBuffers>>) {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::DX12,
            ..InstanceDescriptor::new_without_display_handle()
        });

        let size = window.inner_size().clone();
        let window_bounds = Arc::new(ArcSwap::new(Arc::new(([window.outer_position().expect("idk").x, window.outer_position().expect("idk").y], [window.outer_size().width, window.outer_size().height], false, false))));

        let surface = instance.create_surface(window).expect("Can't create surface");
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            apply_limit_buckets: false
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
            color_space: SurfaceColorSpace::Auto,
        };
        settings.size = size;
        settings.old_size = PhysicalSize::new(0, 0);
        surface.configure(&device, &config);

        let global_textures = GlobalTextures::new(&device);
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
        
        let (buffers, staging_buffers) = Buffers::new(&device, &info, queue.clone(), global_textures.color_texture_view.clone(), global_textures.color_texture_size);
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

                global_textures,
                screen_textures,

                key_handler: Arc::new(ArcSwap::from_pointee(InputHandler::new())),
                cursor_x: Arc::new(AtomicF32::new(0.0)),
                cursor_y: Arc::new(AtomicF32::new(0.0)),
                cursor_locked: Arc::new(AtomicBool::new(false)),
                window_bounds,
                
                globals_updated: Arc::new(AtomicBool::new(false)),
                ticks_updated: Arc::new(AtomicBool::new(false)),
                del_timer: Arc::new(AtomicF64::new(0.0)),
            },
            
            staging_buffers,
        )
    }
}