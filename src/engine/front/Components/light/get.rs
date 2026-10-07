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
}