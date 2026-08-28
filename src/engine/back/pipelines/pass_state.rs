use wgpu::{BindGroup, BindGroupLayout, ComputePipeline, RenderPipeline};

pub struct PassState {
    pub bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout,

    pub pipeline: RenderPipeline,
}

pub struct DoublePassState {
    pub bind_group_layout: BindGroupLayout,
    pub bind_group1: BindGroup,
    pub bind_group2: BindGroup,

    pub pipeline1: RenderPipeline,
    pub pipeline2: RenderPipeline,
}

pub struct SimplePassState {
    pub bind_group_layout: BindGroupLayout,
    pub pipeline: RenderPipeline,
}


pub struct ScreenPassState {
    pub bind_group: BindGroup, 
    pub bind_group_layout: BindGroupLayout,

    pub screen_bind_group: BindGroup,
    pub screen_bind_group_layout: BindGroupLayout,

    pub pipeline: RenderPipeline,
}

pub struct ComputePassState {
    pub bind_group: BindGroup,
    pub pipeline: ComputePipeline,
}