use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, ColorTargetState, PipelineLayoutDescriptor, RenderPipelineDescriptor, SamplerBindingType, ShaderModuleDescriptor, ShaderSource, ShaderStages, SurfaceConfiguration, TextureFormat, TextureSampleType, VertexBufferLayout, VertexStepMode};
use wgpu::TextureViewDimension::D2;
use crate::engine::back::buffers::Buffers;
use crate::engine::back::screen_textures::ScreenTextures;
use crate::engine::back::pipelines::pass_state::{DoublePassState};
use crate::engine::back::pipelines::Pipelines;
use crate::engine::back::shaders::Shaders;

pub fn light_rays(device: &wgpu::Device, config: &SurfaceConfiguration, shaders: &Shaders, buffers: &Buffers, screen_textures: &ScreenTextures) -> DoublePassState {
    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Light rays bind group layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: D2,
                    multisampled: false,
                },
                count: None,
            },
        ],
    });

    let bind_group1 = device.create_bind_group(&BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&screen_textures.light_rays_view),
            },
        ],
        label: Some("Light rays bind group"),
    });

    let bind_group2 = device.create_bind_group(&BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&screen_textures.light_rays_view_scratch),
            },
        ],
        label: Some("Light rays bind group"),
    });


    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Light rays Shader"),
        source: ShaderSource::Wgsl(shaders.light_rays.into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Light rays pipeline layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline1 = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Light rays pipeline"),
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
            entry_point: Some("fs_main_x"),
            compilation_options: Default::default(),
            targets: &[Some(ColorTargetState {
                format: TextureFormat::Rgba16Float,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::One,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::One,
                        operation: wgpu::BlendOperation::Add,
                    },
                }),
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

    let pipeline2 = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Light rays pipeline"),
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
            entry_point: Some("fs_main_y"),
            compilation_options: Default::default(),
            targets: &[Some(ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::One,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::One,
                        operation: wgpu::BlendOperation::Add,
                    },
                }),
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

    DoublePassState {
        bind_group1,
        bind_group2,
        bind_group_layout,

        pipeline1,
        pipeline2,
    }
}

impl Pipelines {
    pub fn remake_light_rays_bind_group(&mut self, device: &wgpu::Device, buffers: &Buffers, screen_textures: &ScreenTextures) {
        self.light_rays_pass.bind_group1 = device.create_bind_group(&BindGroupDescriptor {
            layout: &self.light_rays_pass.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&screen_textures.light_rays_view),
                },
            ],
            label: Some("light rays screen bind group"),
        });

        self.light_rays_pass.bind_group2 = device.create_bind_group(&BindGroupDescriptor {
            layout: &self.light_rays_pass.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&screen_textures.light_rays_view_scratch),
                },
            ],
            label: Some("light rays screen bind group"),
        });
    }
}