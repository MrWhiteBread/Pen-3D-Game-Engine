use crate::engine::front::components::model::{Model, RawModel};

impl RawModel {
    pub(crate) fn rot(&mut self, rot: &[f32; 4], lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let object = scene.get_mut_object(self.id);
                object.rot = *rot;

                if !lerp {
                    object.old_rot = *rot;
                } else {
                    scene.object_changed(self.id);
                }
            }
        } else {
            let object = self.object.as_mut().expect("No object found");

            object.rot = *rot;
            object.old_rot = *rot;
        }
    }
}