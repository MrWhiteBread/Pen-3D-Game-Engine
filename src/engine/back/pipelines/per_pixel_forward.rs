use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, ColorTargetState, PipelineLayoutDescriptor, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, SurfaceConfiguration, TextureFormat, TextureSampleType, VertexBufferLayout, VertexStepMode};
use wgpu::TextureViewDimension::D2;
use crate::engine::back::buffers::Buffers;
use crate::engine::back::screen_textures::ScreenTextures;
use crate::engine::back::pipelines::pass_state::{ScreenPassState};
use crate::engine::back::pipelines::Pipelines;
use crate::engine::back::shaders::Shaders;

pub fn per_pixel_forward(device: &wgpu::Device, buffers: &Buffers, config: &SurfaceConfiguration, shaders: &Shaders, screen_textures: &ScreenTextures) -> ScreenPassState {
    let screen_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Per pixel screen forward bind group layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format: TextureFormat::Rgba16Float,
                    view_dimension: D2,
                },
                count: None,
            },
        ],
    });

    let screen_bind_group = device.create_bind_group(&BindGroupDescriptor {
        layout: &screen_bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&screen_textures.light_rays_view),
            },
        ],
        label: Some("Per pixel screen forward bind group"),
    });


    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Per pixel forward bind group layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 3,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 4,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: buffers.globals.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: buffers.camera_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 2,
                resource: buffers.light_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 3,
                resource: buffers.count_light_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 4,
                resource: buffers.light_ids_buffer.as_entire_binding(),
            },
        ],
        label: Some("Per pixel forward bind group"),
    });


    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Per pixel forward Shader"),
        source: ShaderSource::Wgsl(shaders.per_pixel_forward_rendering.into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Per pixel forward pipeline layout"),
        bind_group_layouts: &[&bind_group_layout, &screen_bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Per pixel forward pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[
                VertexBufferLayout {
                    array_stride: (size_of::<f32>() * 2) as wgpu::BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                }
            ],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            strip_index_format: None,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    ScreenPassState {
        bind_group,
        bind_group_layout,

        screen_bind_group,
        screen_bind_group_layout,

        pipeline,
    }
}

impl Pipelines {
    pub fn remake_pp_forward_screen_bind_group(&mut self, device: &wgpu::Device, screen_textures: &ScreenTextures) {
        self.per_pixel_forward_pass.screen_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &self.per_pixel_forward_pass.screen_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&screen_textures.light_rays_view),
                },
            ],
            label: Some("per pixel forward screen bind group"),
        });
    }
}