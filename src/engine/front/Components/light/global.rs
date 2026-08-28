use crate::engine::front::components::light::Light;

impl Light {
    pub(crate) fn rot(&mut self, rot: &[f32; 3], lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.rotation[0] = rot[0];
                light.rotation[1] = rot[1];
                light.rotation[2] = rot[2];

                if !lerp {
                    light.old_rotation[0] = rot[0];
                    light.old_rotation[1] = rot[1];
                    light.old_rotation[2] = rot[2];
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.rotation[0] = rot[0];
            self.light.rotation[1] = rot[1];
            self.light.rotation[2] = rot[2];

            self.light.old_rotation[0] = rot[0];
            self.light.old_rotation[1] = rot[2];
            self.light.old_rotation[1] = rot[2];
        }
    }

    pub(crate) fn ray_size(&mut self, size: &f32, lerp: &bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let light = scene.get_mut_light(&self.id);
                light.attributes2[1] /= light.attributes2[2] / size;
                light.attributes2[2] = *size;

                if !lerp {
                    light.old_attributes2[1] /= light.attributes2[2] / size;
                    light.old_attributes2[2] = *size;
                } else {
                    scene.light_changed(&self.id);
                }
            }
        } else {
            self.light.attributes2[1] /= self.light.attributes2[2] / size;
            self.light.old_attributes2[1] /= self.light.attributes2[2] / size;

            self.light.attributes2[2] = *size;
            self.light.old_attributes2[2] = *size;
        }
    }
}