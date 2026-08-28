use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub old_color: [f32; 4],
    pub old_position: [f32; 3],
    pub normal: [f32; 3],
    pub old_normal: [f32; 3],
    
    pub material: [f32; 2],
    pub old_material: [f32; 2],
    pub uv: [f32; 2],
    pub old_uv: [f32; 2],
    
    pub skeleton: u32,
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 11] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4, 2 => Float32x4, 3 => Float32x3, 4 => Float32x3, 5 => Float32x3, 6 => Float32x2, 7 => Float32x2, 8 => Float32x2, 9 => Float32x2, 11 => Uint32];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}