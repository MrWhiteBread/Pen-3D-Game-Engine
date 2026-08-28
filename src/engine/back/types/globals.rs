#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub t: f32,
    
    pub scene_light: f32,
    pub old_scene_light: f32,
    pub lerp_scene_light: f32,
    
    pub screen_size: [f32; 2],
    pub padding: [f32; 2],
}