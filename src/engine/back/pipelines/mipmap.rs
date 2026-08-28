use wgpu::{BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, ColorTargetState, PipelineLayoutDescriptor, RenderPipelineDescriptor, SamplerBindingType, ShaderModuleDescriptor, ShaderSource, ShaderStages, SurfaceConfiguration, TextureFormat, TextureSampleType, VertexBufferLayout, VertexStepMode};
use wgpu::TextureViewDimension::D2;
use crate::engine::back::buffers::Buffers;
use crate::engine::back::screen_textures::ScreenTextures;
use crate::engine::back::pipelines::pass_state::{SimplePassState};
use crate::engine::back::shaders::Shaders;

pub fn mipmap(device: &wgpu::Device, shaders: &Shaders) -> SimplePassState {
    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Mipmap bind group layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });


    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Mipmap Shader"),
        source: ShaderSource::Wgsl(shaders.mipmap.into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Mipmap pipeline layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Mipmap pipeline"),
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

    SimplePassState {
        bind_group_layout,

        pipeline,
    }
}