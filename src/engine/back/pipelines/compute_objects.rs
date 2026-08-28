use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, PipelineLayoutDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages};
use crate::engine::back::buffers::Buffers;
use crate::engine::back::pipelines::pass_state::{ComputePassState};
use crate::engine::back::shaders::Shaders;

pub fn compute_objects(device: &wgpu::Device, buffers: &Buffers, shaders: &Shaders) -> ComputePassState {
    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("compute objects bind group layout"),
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
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None
            },
            BindGroupLayoutEntry {
                binding: 3,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 4,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 5,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 6,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
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
                resource: buffers.command_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: buffers.count_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 2,
                resource: buffers.object_ids_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 3,
                resource: buffers.globals.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 4,
                resource: buffers.camera_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 5,
                resource: buffers.object_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 6,
                resource: buffers.object_count_buffer.as_entire_binding(),
            },
        ],
        label: Some("Compute objects Bind Group"),
    });

    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Compute objects Shader"),
        source: ShaderSource::Wgsl(shaders.objects.into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Compute objects Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute objects Pipeline"),
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