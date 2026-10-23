use std::cell::Ref;
use glam::Quat;
use crate::engine::front::components::mesh::Mesh;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Object {
    pub rot: [f32; 4],
    pub old_rot: [f32; 4],

    pub center: [f32; 4], // material[0]
    pub old_center: [f32; 4], 

    pub size: [f32; 4], // material[1]
    pub old_size: [f32; 4],

    pub color: u32,
    pub old_color: u32,

    pub id: u32,

    pub index_count: u32,
    pub first_index: u32,
    pub base_vertex: i32,
    pub radius: f32,

    pub padding: u32,
}

impl Object {
    pub fn new(center: &[f32; 3], rot: &[f32; 3], size: &[f32; 3], color: [u8; 3], material: &[f32; 2]) -> Self {
        let color = u32::from_le_bytes([color[0], color[1], color[2], 255]); // color[3] is state, 0 don t render, 255 render

        let x = Quat::from_rotation_x(rot[0].to_radians());
        let y = Quat::from_rotation_y(rot[1].to_radians());
        let z = Quat::from_rotation_z(rot[2].to_radians());
        let rot = (x * y * z).to_array();

        Self {
            rot,
            old_rot: rot,

            center: [center[0], center[1], center[2], material[0]],
            old_center: [center[0], center[1], center[2], material[0]],

            size: [size[0], size[1], size[2], material[1]],
            old_size: [size[0], size[1], size[2], material[1]],

            color,
            old_color: color,

            id: 0,

            index_count: 0,
            first_index: 0,
            base_vertex: 0,
            radius: 0.0,

            padding: 0,
        }
    }
}