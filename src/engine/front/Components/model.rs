use glam::{EulerRot, Quat};
use crate::engine::back::render::scene::Scene;
use crate::engine::back::types::object::Object;
use crate::engine::front::components::mesh::Mesh;

pub struct Model {
    scene: Option<Scene>,

    object: Object,
    mesh: Option<Mesh>,
    angle: [f32; 3],

    id: u32,
}

#[allow(dead_code)]
impl Model {
    pub fn new(mut object: Object, mesh: Mesh) -> Self {
        object.set_mesh(&mesh);
        let angle = Quat::from_array(object.rot).to_euler(EulerRot::XYZ);
        let angle = [angle.0.to_degrees(), angle.1.to_degrees(), angle.2.to_degrees()];
        
        Self {
            object,
            mesh: Some(mesh),
            scene: None,
            angle,
            
            id: 0,
        }
    }

    pub fn get_pos(&self) -> [f32; 3] {
        if let Some(scene) = &self.scene {
            unsafe {
                let object = scene.get_object(&self.id);
                [object.center[0], object.center[1], object.center[2]]
            }
        } else {
            [self.object.center[0], self.object.center[1], self.object.center[2]]
        }
    }

    pub fn get_raw_rot(&self) -> [f32; 4] {
        if let Some(scene) = &self.scene {
            unsafe {
                let object = scene.get_object(&self.id);
                object.rot
            }
        } else {
            self.object.rot
        }
    }

    pub fn get_rot(&self) -> [f32; 3] {
        self.angle
    }

    pub fn get_size(&self) -> [f32; 3] {
        if let Some(scene) = &self.scene {
            unsafe {
                let object = scene.get_object(&self.id);
                [object.size[0], object.size[1], object.size[2]]
            }
        } else {
            [self.object.size[0], self.object.size[1], self.object.size[2]]
        }
    }

    pub fn change_pos(&mut self, pos: &[f32; 3], lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let object = scene.get_mut_object(&self.id);
                object.center[0] += pos[0];
                object.center[1] += pos[1];
                object.center[2] += pos[2];

                if !lerp {
                    object.old_center[0] += pos[0];
                    object.old_center[1] += pos[1];
                    object.old_center[2] += pos[2];
                } else {
                    scene.object_changed(&self.id);
                }
            }

            scene.verify_model(&self.id);
        } else {
            self.object.center[0] += pos[0];
            self.object.center[1] += pos[1];
            self.object.center[2] += pos[2];

            self.object.old_center[0] += pos[0];
            self.object.old_center[1] += pos[1];
            self.object.old_center[2] += pos[2];
        }
    }

    pub fn set_pos(&mut self, pos: &[f32; 3], lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let object = scene.get_mut_object(&self.id);
                object.center[0] = pos[0];
                object.center[1] = pos[1];
                object.center[2] = pos[2];

                if !lerp {
                    object.old_center[0] = pos[0];
                    object.old_center[1] = pos[1];
                    object.old_center[2] = pos[2];
                } else {
                    scene.object_changed(&self.id);
                }
            }

            scene.verify_model(&self.id);
        } else {
            self.object.center[0] = pos[0];
            self.object.center[1] = pos[1];
            self.object.center[2] = pos[2];

            self.object.old_center[0] = pos[0];
            self.object.old_center[1] = pos[1];
            self.object.old_center[2] = pos[2];
        }
    }

    fn rot(&mut self, rot: &[f32; 4], lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let object = scene.get_mut_object(&self.id);
                object.rot = *rot;

                if !lerp {
                    object.old_rot = *rot;
                } else {
                    scene.object_changed(&self.id);
                }
            }
        } else {
            self.object.rot = *rot;
            self.object.old_rot = *rot;
        }
    }

    pub fn change_rot(&mut self, rot: &[f32; 3], lerp: bool) {
        self.angle[0] += rot[0];
        self.angle[1] += rot[1];
        self.angle[2] += rot[2];

        let x = Quat::from_rotation_x(self.angle[0].to_radians());
        let y = Quat::from_rotation_y(self.angle[1].to_radians());
        let z = Quat::from_rotation_z(self.angle[2].to_radians());
        let rot = (x * y * z).to_array();

        self.rot(&rot, lerp);
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

    pub fn change_size(&mut self, size: &[f32; 3], lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let object = scene.get_mut_object(&self.id);
                object.size[0] += size[0];
                object.size[1] += size[1];
                object.size[2] += size[2];

                if !lerp {
                    object.old_size[0] += size[0];
                    object.old_size[1] += size[1];
                    object.old_size[2] += size[2];
                } else {
                    scene.object_changed(&self.id);
                }
            }
        } else {
            self.object.size[0] += size[0];
            self.object.size[1] += size[1];
            self.object.size[2] += size[2];

            self.object.old_size[0] += size[0];
            self.object.old_size[1] += size[1];
            self.object.old_size[2] += size[2];
        }
    }

    pub fn set_size(&mut self, size: &[f32; 3], lerp: bool) {
        if let Some(scene) = &mut self.scene {
            unsafe {
                let object = scene.get_mut_object(&self.id);
                object.size[0] = size[0];
                object.size[1] = size[1];
                object.size[2] = size[2];

                if !lerp {
                    object.old_size[0] = size[0];
                    object.old_size[1] = size[1];
                    object.old_size[2] = size[2];
                } else {
                    scene.object_changed(&self.id);
                }
            }
        } else {
            self.object.size[0] = size[0];
            self.object.size[1] = size[1];
            self.object.size[2] = size[2];

            self.object.old_size[0] = size[0];
            self.object.old_size[1] = size[1];
            self.object.old_size[2] = size[2];
        }
    }

    pub fn load(&mut self, scene: &Scene) {
        if self.scene.is_none() && let Some(mesh) = self.mesh.take() {
            self.id = scene.add_model(self.object, mesh);
            self.scene = Some(scene.clone());
        }
    }
}