use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, PipelineLayoutDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages};
use crate::engine::back::buffers::Buffers;
use crate::engine::back::pipelines::pass_state::{ComputePassState};
use crate::engine::back::shaders::Shaders;

pub fn compute_once(device: &wgpu::Device, buffers: &Buffers, shaders: &Shaders) -> ComputePassState {
    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("compute once bind group layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: false },
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
        ],
        label: Some("Compute once Bind Group"),
    });

    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Compute once Shader"),
        source: ShaderSource::Wgsl(shaders.once.into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Compute once Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute once Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("compute_main"),
        compilation_options: Default::default(),
        cache: None,
    });

    ComputePassState {
        bind_group,
        pipeline,
    }
}