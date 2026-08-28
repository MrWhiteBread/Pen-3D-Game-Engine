use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, ColorTargetState, PipelineLayoutDescriptor, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureFormat, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use wgpu::TextureFormat::{Rg16Float, Rgba16Float, Rgba32Float};
use crate::engine::back::buffers::Buffers;
use crate::engine::back::pipelines::pass_state::PassState;
use crate::engine::back::shaders::Shaders;
use crate::engine::back::types::vertex::Vertex;

pub fn deferred_v(device: &wgpu::Device, buffers: &Buffers, shaders: &Shaders) -> PassState {
    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Deferred vertex bind group layout"),
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
        ],
        label: Some("Deferred vertex global bind group"),
    });

    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Deferred vertex shader"),
        source: ShaderSource::Wgsl(shaders.deferred_vertex.into()),
    });

    let pipeline_layout_ = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Deferred vertex pipeline layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Deferred vertex pipeline"),
        layout: Some(&pipeline_layout_),
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
            targets: &[
                Some(ColorTargetState {
                    format: Rgba32Float,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                }),
                Some(ColorTargetState {
                    format: Rgba16Float,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                }),
                Some(ColorTargetState {
                    format: Rg16Float,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })
            ],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            strip_index_format: None,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    PassState {
        bind_group,
        bind_group_layout,
        pipeline,
    }
}