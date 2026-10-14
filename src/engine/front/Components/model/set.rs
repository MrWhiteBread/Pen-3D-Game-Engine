use glam::Quat;
use crate::engine::front::components::mesh::Mesh;
use crate::engine::front::components::model::Model;

impl Model {
    pub fn set_mesh(&mut self, mesh: &mut Mesh) {
        self.mesh = mesh;

        if let Some(scene) = self.scene.as_ref() {
            unsafe {
                scene.set_object_mesh(self.id, (*self.mesh).get_ids());
                let object = scene.get_mut_object(self.id);
                object.set_mesh(mesh);

                self.mesh_id = (*self.mesh)._add_loaded_model(self);
            }

        } else if let Some(object) = self.object.as_mut() {
            object.set_mesh(mesh);
            self.mesh_id = mesh._add_unloaded_model(self);
        }
    }
    
    pub fn set_pos(&mut self, pos: &[f32; 3], lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let object = scene.get_mut_object(self.id);
                object.center[0] = pos[0];
                object.center[1] = pos[1];
                object.center[2] = pos[2];

                if !lerp {
                    object.old_center[0] = pos[0];
                    object.old_center[1] = pos[1];
                    object.old_center[2] = pos[2];
                } else {
                    scene.object_changed(self.id);
                }
            }

            scene.verify_model(self.id);
        } else {
            let object = self.object.as_mut().expect("No object found");

            object.center[0] = pos[0];
            object.center[1] = pos[1];
            object.center[2] = pos[2];

            object.old_center[0] = pos[0];
            object.old_center[1] = pos[1];
            object.old_center[2] = pos[2];
        }
    }

    pub fn set_rot(&mut self, rot: &[f32; 3], lerp: bool) {
        self.angle[0] = rot[0];
        self.angle[1] = rot[1];
        self.angle[2] = rot[2];

        let x = Quat::from_rotation_x(rot[0].to_radians());
        let y = Quat::from_rotation_y(rot[1].to_radians());
        let z = Quat::from_rotation_z(rot[2].to_radians());
        let rot = (x * y * z).to_array();

        self.rot(&rot, lerp);
    }

    pub fn set_size(&mut self, size: &[f32; 3], lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let object = scene.get_mut_object(self.id);
                object.size[0] = size[0];
                object.size[1] = size[1];
                object.size[2] = size[2];

                if !lerp {
                    object.old_size[0] = size[0];
                    object.old_size[1] = size[1];
                    object.old_size[2] = size[2];
                } else {
                    scene.object_changed(self.id);
                }
            }
        } else {
            let object = self.object.as_mut().expect("No object found");

            object.size[0] = size[0];
            object.size[1] = size[1];
            object.size[2] = size[2];

            object.old_size[0] = size[0];
            object.old_size[1] = size[1];
            object.old_size[2] = size[2];
        }
    }
}