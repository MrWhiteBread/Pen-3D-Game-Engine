use glam::Quat;
use crate::engine::front::components::mesh::{Mesh, RawMesh};
use crate::engine::front::components::model::{Model, RawModel};

impl Model {
    #[track_caller]
    pub fn set_chunk(&mut self, id: u32) {
        unsafe {
            (*self.model).set_chunk(id);
        }
    }

    pub fn set_mesh(&mut self, mesh: &mut Mesh) {
        unsafe {
            (*self.model).set_mesh(mesh.get_raw_mesh());
        }
    }

    pub fn set_pos(&mut self, pos: &[f32; 3], lerp: bool) {
        unsafe {
            (*self.model).set_pos(pos, lerp);
        }
    }

    pub fn set_rot(&mut self, rot: &[f32; 3], lerp: bool) {
        unsafe {
            (*self.model).set_rot(rot, lerp);
        }
    }

    pub fn set_size(&mut self, size: &[f32; 3], lerp: bool) {
        unsafe {
            (*self.model).set_size(size, lerp);
        }
    }
}

impl RawModel {
    #[track_caller]
    pub fn set_chunk(&mut self, id: u32) {
        if let Some(scene) = self.scene.as_mut() {
            scene.set_object_chunk(self.id, id);
        } else {
            panic!("Can't set model chunk because it doesn't hold a scene")
        }
    }

    pub fn set_mesh(&mut self, mesh: *mut RawMesh) {
        if !self.mesh.is_null() {
            unsafe {
                if self.scene.is_some() {
                    (*self.mesh)._remove_loaded_model(self.mesh_id);
                } else {
                    (*self.mesh)._remove_unloaded_model(self.mesh_id);
                }
            }
        }

        self.mesh = mesh;

        if let Some(scene) = self.scene.as_ref() {
            unsafe {
                scene.set_object_mesh(self.id, (*self.mesh).get_ids());

                let mut object = scene.get_mut_object(self.id);
                Self::set_obj_mesh(&mut object, mesh);

                self.mesh_id = (*self.mesh)._add_loaded_model(self);
            }

        } else {
            let mut object = self.object.as_mut().expect("empty object");
            
            unsafe {
                Self::set_obj_mesh(&mut object, mesh);
                self.mesh_id = (*mesh)._add_unloaded_model(self);
            }
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