use crate::engine::front::components::light::Light;

impl Light {
    pub fn get_pos(&self) -> [f32; 3] {
        if let Some(scene) = &self.scene {
            unsafe {
                let light = scene.get_light(&self.id);
                [light.position[0], light.position[1], light.position[2]]
            }
        } else {
            [self.light.position[0], self.light.position[1], self.light.position[2]]
        }
    }

    pub fn get_raw_rot(&self) -> [f32; 3] {
        if let Some(scene) = &self.scene {
            unsafe {
                let light = scene.get_light(&self.id);
                [light.rotation[0], light.rotation[1], light.rotation[2]]
            }
        } else {
            [self.light.rotation[0], self.light.rotation[1], self.light.rotation[2]]
        }
    }

    pub fn get_rot(&self) -> [f32; 3] {
        self.angle
    }

    pub fn get_color(&self) -> [f32; 4] {
        if let Some(scene) = &self.scene {
            unsafe {
                scene.get_light(&self.id).color
            }
        } else {
            self.light.color
        }
    }

    pub fn get_softness(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                scene.get_light(&self.id).attributes[0]
            }
        } else {
            self.light.attributes[0]
        }
    }

    pub fn get_range(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                scene.get_light(&self.id).attributes[1]
            }
        } else {
            self.light.attributes[1]
        }
    }

    pub fn get_fov(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                scene.get_light(&self.id).attributes[2]
            }
        } else {
            self.light.attributes[2]
        }
    }


    pub fn get_rays_intensity(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                scene.get_light(&self.id).attributes[3]
            }
        } else {
            self.light.attributes[3]
        }
    }

    pub fn get_rays_range(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                scene.get_light(&self.id).attributes2[0]
            }
        } else {
            self.light.attributes2[0]
        }
    }

    pub fn get_rays_fog_range(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                scene.get_light(&self.id).attributes2[3]
            }
        } else {
            self.light.attributes2[3]
        }
    }

    pub fn get_rays_count(&self) -> u32 {
        if let Some(scene) = &self.scene {
            unsafe {
                let light = scene.get_light(&self.id);
                (light.attributes2[1] / light.attributes2[2]) as u32
            }
        } else {
            (self.light.attributes2[1] / self.light.attributes2[2]) as u32
        }
    }

    pub fn get_rays_size(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                let light = scene.get_light(&self.id);
                1.0 / light.attributes2[2]
            }
        } else {
            1.0 / self.light.attributes2[2]
        }
    }

    pub fn get_rays_softness(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                let light = scene.get_light(&self.id);
                light.attributes3[0]
            }
        } else {
            self.light.attributes3[0]
        }
    }

    pub fn get_rays_fog_softness(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                let light = scene.get_light(&self.id);
                light.attributes3[1]
            }
        } else {
            self.light.attributes3[1]
        }
    }

    pub fn get_rays_fog_intensity(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                let light = scene.get_light(&self.id);
                light.attributes3[2]
            }
        } else {
            self.light.attributes3[2]
        }
    }

    pub fn get_rays_r_angle_roughness(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                let light = scene.get_light(&self.id);
                light.attributes4[0]
            }
        } else {
            self.light.attributes4[0]
        }
    }

    pub fn get_rays_r_empty_delta(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                let light = scene.get_light(&self.id);
                light.attributes4[0]
            }
        } else {
            self.light.attributes4[0]
        }
    }

    pub fn get_rays_r_angle_mod(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                let light = scene.get_light(&self.id);
                light.attributes4[2]
            }
        } else {
            self.light.attributes4[2]
        }
    }

    pub fn get_rays_r_angle_div(&self) -> f32 {
        if let Some(scene) = &self.scene {
            unsafe {
                let light = scene.get_light(&self.id);
                light.attributes4[3]
            }
        } else {
            self.light.attributes4[3]
        }
    }
}