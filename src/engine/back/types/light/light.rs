#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub position: [f32; 4],
    pub rotation: [f32; 4],

    pub color: [f32; 4],
    pub attributes: [f32; 4],
}