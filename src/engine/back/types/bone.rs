#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Bone {
    pub matrix: [[f32; 4]; 4],
}