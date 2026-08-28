use glam::{Quat, Vec3};
use crate::engine::front::components::light::Light;

impl Light {
    pub fn change_rot(&mut self, rot: &[f32; 3], lerp: &bool) {
        self.angle[0] += rot[0];
        self.angle[1] += rot[1];
        self.angle[2] += rot[2];

        let x = Quat::from_rotation_x(self.angle[0].to_radians());
        let y = Quat::from_rotation_y(self.angle[1].to_radians());
        let z = Quat::from_rotation_z(self.angle[2].to_radians());
        let rot = ((x * y * z) * Vec3::new(0.0, -1.0, 0.0)).to_array();

        self.rot(&rot, lerp);
    }

    pub fn change_pos(&mut self, pos: &[f32; 3], lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.position[0] += pos[0];
                light.position[1] += pos[1];
                light.position[2] += pos[2];

                if !lerp {
                    light.old_position[0] += pos[0];
                    light.old_position[1] += pos[1];
                    light.old_position[2] += pos[2];
                } else {
                    scene.light_changed(&self.id);
                }
            }

            scene.verify_light(&self.id);
        } else {
            self.light.position[0] += pos[0];
            self.light.position[1] += pos[1];
            self.light.position[2] += pos[2];

            self.light.old_position[0] += pos[0];
            self.light.old_position[1] += pos[1];
            self.light.old_position[2] += pos[2];
        }
    }

    pub fn change_color(&mut self, color: &[f32; 4], lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
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
                    scene.light_changed(&self.id);
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

    pub fn change_softness(&mut self, softness: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes[0] += softness;

                if !lerp {
                    light.old_attributes[0] += softness;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes[0] += softness;
            self.light.old_attributes[0] += softness;
        }
    }

    pub fn change_range(&mut self, range: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes[1] += range;

                if !lerp {
                    light.old_attributes[1] += range;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes[1] += range;
            self.light.old_attributes[1] += range;
        }
    }

    pub fn change_fov(&mut self, fov: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes[2] += fov;

                if !lerp {
                    light.old_attributes[2] += fov;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes[2] += fov;
            self.light.old_attributes[2] += fov;
        }
    }
    

    pub fn change_rays_intensity(&mut self, intensity: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes[3] += intensity;

                if !lerp {
                    light.old_attributes[3] += intensity;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes[3] += intensity;
            self.light.old_attributes[3] += intensity;
        }
    }

    pub fn change_rays_range(&mut self, range: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes2[0] += range;

                if !lerp {
                    light.old_attributes2[0] += range;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes2[0] += range;
            self.light.old_attributes2[0] += range;
        }
    }

    pub fn change_rays_fog_range(&mut self, range: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes2[3] += range;

                if !lerp {
                    light.old_attributes2[3] += range;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes2[3] += range;
            self.light.old_attributes2[3] += range;
        }
    }

    pub fn change_rays_count(&mut self, count: &u32, lerp: &bool) {
        self.set_rays_count(&(count + self.get_rays_count()), lerp);
    }

    pub fn change_rays_size(&mut self, size: &f32, lerp: &bool) {
        let size = 1.0 / (size + 1.0 / self.light.attributes2[2]);
        self.ray_size(&size, lerp);
    }

    pub fn change_rays_softness(&mut self, softness: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes3[0] += softness;

                if !lerp {
                    light.old_attributes3[0] += softness;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes3[0] += softness;
            self.light.old_attributes3[0] += softness;
        }
    }

    pub fn change_rays_fog_softness(&mut self, softness: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes3[1] += softness;

                if !lerp {
                    light.old_attributes3[1] += softness;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes3[1] += softness;
            self.light.old_attributes3[1] += softness;
        }
    }

    pub fn change_rays_fog_intensity(&mut self, intensity: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes3[2] += intensity;

                if !lerp {
                    light.old_attributes3[2] += intensity;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes3[2] += intensity;
            self.light.old_attributes3[2] += intensity;
        }
    }

    pub fn change_rays_r_angle_roughness(&mut self, roughness: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes4[0] += roughness;

                if !lerp {
                    light.old_attributes4[0] += roughness;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes4[0] += roughness;
            self.light.old_attributes4[0] += roughness;
        }
    }

    pub fn change_rays_r_empty_delta(&mut self, delta: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes4[1] += delta;

                if !lerp {
                    light.old_attributes4[1] += delta;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes4[1] += delta;
            self.light.old_attributes4[1] += delta;
        }
    }

    pub fn change_rays_r_angle_mod(&mut self, modulo: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes4[2] += modulo;

                if !lerp {
                    light.old_attributes4[2] += modulo;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes4[2] += modulo;
            self.light.old_attributes4[2] += modulo;
        }
    }

    pub fn change_rays_r_angle_div(&mut self, div: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes4[3] += div;

                if !lerp {
                    light.old_attributes4[3] += div;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes4[3] += div;
            self.light.old_attributes4[3] += div;
        }
    }
}