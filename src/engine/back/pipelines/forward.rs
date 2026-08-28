use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, ColorTargetState, PipelineLayoutDescriptor, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, SurfaceConfiguration, TextureFormat, TextureSampleType, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use wgpu::TextureViewDimension::D2;
use crate::engine::back::buffers::Buffers;
use crate::engine::back::pipelines::pass_state::{ScreenPassState};
use crate::engine::back::pipelines::Pipelines;
use crate::engine::back::screen_textures::ScreenTextures;
use crate::engine::back::shaders::Shaders;
use crate::engine::back::types::vertex::Vertex;

pub fn forward(device: &wgpu::Device, buffers: &Buffers, config: &SurfaceConfiguration, shaders: &Shaders, screen_textures: &ScreenTextures) -> ScreenPassState {
    let screen_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("forward screen bind group layout"),
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
        label: Some("forward screen bind group"),
    });
    
    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("forward bind group layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::VERTEX,
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
            BindGroupLayoutEntry {
                binding: 5,
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
                resource: buffers.object_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 3,
                resource: buffers.light_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 4,
                resource: buffers.count_light_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 5,
                resource: buffers.light_ids_buffer.as_entire_binding(),
            },
        ],
        label: Some("forward bind group"),
    });

    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("forward shader"),
        source: ShaderSource::Wgsl(shaders.forward_rendering.into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("forward render pipeline layout"),
        bind_group_layouts: &[&bind_group_layout, &screen_bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("forward render pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[
                Vertex::desc(),
                VertexBufferLayout {
                    array_stride: 4,
                    step_mode: VertexStepMode::Instance,
                    attributes: &[VertexAttribute {
                        offset: 0,
                        shader_location: 10,
                        format: VertexFormat::Uint32,
                    }]
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
        depth_stencil: Some(wgpu::DepthStencilState {
            format: TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    ScreenPassState{
        bind_group,
        bind_group_layout,
        
        screen_bind_group,
        screen_bind_group_layout,
        
        pipeline,
    }
}

impl Pipelines {
    pub fn remake_forward_screen_bind_group(&mut self, device: &wgpu::Device, screen_textures: &ScreenTextures) {
        self.forward_pass.screen_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &self.forward_pass.screen_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&screen_textures.light_rays_view),
                },
            ],
            label: Some("forward screen bind group"),
        });
    }
}