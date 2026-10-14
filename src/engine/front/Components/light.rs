mod add;
mod get;
mod set;
mod global;

use glam::{EulerRot, Quat, Vec3};
use crate::engine::back::render::scene::Scene;
use crate::engine::back::types::light::LightType;
pub struct Light {
    scene: Option<Scene>,
    light: Option<LightType>,
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
            light: Some(LightType::new(position, rot, color, attributes)),
            angle: rotation,

            id: 0,
        }
    }

    #[track_caller]
    pub fn load(&mut self, scene: &Scene) {
        if self.scene.is_none() && let Some(light) = self.light.take() {
            self.id = scene.add_light(light);
            self.scene = Some(scene.clone());
        } else {
            panic!("Light is already loaded");
        }
    }

    #[track_caller]
    pub fn unload(&mut self) {
        if let Some(scene) = self.scene.as_mut() && self.light.is_none(){
            self.light = Some(scene.remove_light(self.id));
        } else {
            panic!("Light is already unloaded");
        }
    }
}

impl Drop for Light {
    fn drop(&mut self) {
        if self.light.is_some() {
            return;
        }
        self.unload();
    }
}