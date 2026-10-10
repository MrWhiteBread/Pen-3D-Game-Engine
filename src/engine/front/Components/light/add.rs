use glam::{Quat, Vec3};
use crate::engine::back::types::light::LightType;
use crate::engine::front::components::light::Light;

impl Light {
    pub fn get_raw_light(&self) -> &LightType {
        if let Some(scene) = &self.scene {
            unsafe {
                scene.get_light(self.id)
            }
        } else {
            &self.light
        }
    }

    pub fn add_rot(&mut self, rot: &[f32; 3], lerp: bool) {
        self.angle[0] += rot[0];
        self.angle[1] += rot[1];
        self.angle[2] += rot[2];

        let x = Quat::from_rotation_x(self.angle[0].to_radians());
        let y = Quat::from_rotation_y(self.angle[1].to_radians());
        let z = Quat::from_rotation_z(self.angle[2].to_radians());
        let rot = ((x * y * z) * Vec3::new(0.0, -1.0, 0.0)).to_array();

        self.rot(&rot, lerp);
    }

    pub fn add_pos(&mut self, pos: &[f32; 3], lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(self.id);
                light.position[0] += pos[0];
                light.position[1] += pos[1];
                light.position[2] += pos[2];

                if !lerp {
                    light.old_position[0] += pos[0];
                    light.old_position[1] += pos[1];
                    light.old_position[2] += pos[2];
                } else {
                    scene.light_changed(self.id);
                }
            }

            scene.verify_light(self.id);
        } else {
            self.light.position[0] += pos[0];
            self.light.position[1] += pos[1];
            self.light.position[2] += pos[2];

            self.light.old_position[0] += pos[0];
            self.light.old_position[1] += pos[1];
            self.light.old_position[2] += pos[2];
        }
    }

    pub fn add_color(&mut self, color: &[f32; 4], lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(self.id);
                light.color[0] += color[0];
                light.color[1] += color[1];
                light.color[2] += color[2];
                light.color[3] += color[3];

                if !lerp {
                    light.old_color[0] += color[0];
                    light.old_color[1] += color[1];
                    light.old_color[2] += color[2];
                    light.old_color[3] += color[3];
                } else {
                    scene.light_changed(self.id);
                }
            }
        } else {
            self.light.color[0] += color[0];
            self.light.color[1] += color[1];
            self.light.color[2] += color[2];
            self.light.color[3] += color[3];

            self.light.old_color[0] += color[0];
            self.light.old_color[1] += color[1];
            self.light.old_color[2] += color[2];
            self.light.old_color[3] += color[3];
        }
    }

    pub fn add_softness(&mut self, softness: f32, lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(self.id);
                light.attributes[0] += softness;

                if !lerp {
                    light.old_attributes[0] += softness;
                } else {
                    scene.light_changed(self.id);
                }
            }
        } else {
            self.light.attributes[0] += softness;
            self.light.old_attributes[0] += softness;
        }
    }

    pub fn add_range(&mut self, range: f32, lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(self.id);
                light.attributes[1] += range;

                if !lerp {
                    light.old_attributes[1] += range;
                } else {
                    scene.light_changed(self.id);
                }
            }
        } else {
            self.light.attributes[1] += range;
            self.light.old_attributes[1] += range;
        }
    }

    pub fn add_fov(&mut self, fov: f32, lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(self.id);
                light.attributes[2] += fov;

                if !lerp {
                    light.old_attributes[2] += fov;
                } else {
                    scene.light_changed(self.id);
                }
            }
        } else {
            self.light.attributes[2] += fov;
            self.light.old_attributes[2] += fov;
        }
    }
}