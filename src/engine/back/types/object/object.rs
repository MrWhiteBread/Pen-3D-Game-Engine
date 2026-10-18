#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Object {
    rot_mat1: [f32; 4],
    rot_mat2: [f32; 4],
    rot_mat3: [f32; 4],

    center: [f32; 4],
    size: [f32; 4],
}