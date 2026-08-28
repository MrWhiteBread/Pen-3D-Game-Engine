use glam::Quat;
use crate::engine::front::components::mesh::Mesh;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Object {
    pub rot: [f32; 4],
    pub old_rot: [f32; 4],

    pub center: [f32; 4],
    pub lerp_center: [f32; 4],
    pub old_center: [f32; 4],

    pub size: [f32; 4],
    pub lerp_size: [f32; 4],
    pub old_size: [f32; 4],

    pub status: u32,
    pub texture_id: u32,

    pub index_count: u32,
    pub first_index: u32,
    pub base_vertex: i32,
    pub radius: f32,
    pub skeleton: u32,

    pub padding: f32,
    pub rot_mat: [[f32; 4]; 3],
}

impl Object {
    pub fn new(center: &[f32; 3], rot: &[f32; 3], size: &[f32; 3]) -> Self {
        let center = [center[0], center[1], center[2], 1.0];
        
        let x = Quat::from_rotation_x(rot[0].to_radians());
        let y = Quat::from_rotation_y(rot[1].to_radians());
        let z = Quat::from_rotation_z(rot[2].to_radians());
        let rot = (x * y * z).to_array();

        Self {
            rot,
            center,
            
            lerp_size: [0.0; 4],
            lerp_center: [0.0; 4],

            old_rot: rot,
            old_center: center,

            size: [size[0], size[1], size[2], 1.0],
            old_size: [size[0], size[1], size[2], 1.0],

            texture_id: 0,
            status: 1,
            
            index_count: 0,
            first_index: 0,
            base_vertex: 0,
            radius: 0.0,
            skeleton: 0,

            padding: 0.0,
            rot_mat: [[0.0; 4]; 3],
        }
    }
    
    pub fn set_mesh(&mut self, mesh: &Mesh) {
        self.radius = mesh.radius;
    }
}