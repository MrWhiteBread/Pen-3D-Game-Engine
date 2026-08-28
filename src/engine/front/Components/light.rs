mod change;
mod get;
mod set;
mod global;

use glam::{EulerRot, Quat, Vec3};
use crate::engine::back::render::scene::Scene;
use crate::engine::back::types::light::LightType;
pub struct Light {
    scene: Option<Scene>,
    light: LightType,
    angle: [f32; 3],

    id: u32,
}

#[allow(dead_code)]
impl Light {
    pub fn new(position: [f32; 3], rotation: [f32; 3], color: [f32; 4], softness: f32, range: f32, fov: f32) -> Self {
        let attributes = [softness, range, fov, range];
        let x = Quat::from_rotation_x(rotation[0].to_radians());
        let y = Quat::from_rotation_y(rotation[1].to_radians());
        let z = Quat::from_rotation_z(rotation[2].to_radians());
        let rot = ((x * y * z) * Vec3::new(0.0, -1.0, 0.0)).to_array();
        
        Self {
            scene: None,
            light: LightType::new(position, rot, color, attributes),
            angle: rotation,

            id: 0,
        }
    }
    
    pub fn load(&mut self, scene: &Scene) {
        if self.scene.is_none() {
            self.id = scene.add_light(self.light);
            self.scene = Some(scene.clone());
        }
    }
}