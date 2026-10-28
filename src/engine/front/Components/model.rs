mod get;
mod set;
mod add;
mod global;

use std::ptr::null_mut;
use glam::{EulerRot, Quat};
use crate::engine::back::render::scene::Scene;
use crate::engine::back::types::object::raw_object::Object;
use crate::engine::front::components::mesh::{Mesh, RawMesh};

type Id = u32;

#[repr(transparent)]
pub struct Model {
    model: *mut RawModel,
}

impl Model {
    pub fn new(object: Object) -> Self {
        Self {
            model: Box::into_raw(Box::new(RawModel::new(object))),
        }
    }

    pub fn from(object: Object, mesh: &mut Mesh) -> Self {
        Self {
            model: Box::into_raw(Box::new(RawModel::from(object, mesh.get_raw_mesh()))),
        }
    }

    pub fn is_loaded(&self) -> bool {
        unsafe {
            (*self.model).is_loaded()
        }
    }

    #[track_caller]
    pub fn load(&mut self, scene: &Scene) {
        unsafe {
            (*self.model).load(scene);
        }
    }

    #[track_caller]
    pub fn unload(&mut self) {
        unsafe {
            (*self.model).unload();
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.model));
        }
    }
}


pub struct RawModel {
    scene: Option<Scene>,

    object: Option<Object>,
    mesh: *mut RawMesh,
    angle: [f32; 3],

    mesh_id: Id,
    id: Id,
}

#[allow(dead_code)]
impl RawModel {
    pub fn new(object: Object) -> Self {
        let angle = Quat::from_array(object.rot).to_euler(EulerRot::XYZ);
        let angle = [angle.0.to_degrees(), angle.1.to_degrees(), angle.2.to_degrees()];

        Self {
            scene: None,

            object: Some(object),
            mesh: null_mut(),
            angle,

            mesh_id: 0,
            id: 0,
        }
    }

    pub fn from(mut object: Object, mesh: *mut RawMesh) -> Self {
        unsafe {
            Self::set_obj_mesh(&mut object, mesh);
        }

        let angle = Quat::from_array(object.rot).to_euler(EulerRot::XYZ);
        let angle = [angle.0.to_degrees(), angle.1.to_degrees(), angle.2.to_degrees()];

        let mut _self = Self {
            scene: None,

            object: Some(object),
            mesh,
            angle,

            mesh_id: 0,
            id: 0,
        };

        unsafe {
            _self.mesh_id = (*mesh)._add_unloaded_model(&mut _self);
        }

        _self
    }

    unsafe fn set_obj_mesh(object: &mut Object, mesh: *const RawMesh) {
        unsafe {
            object.radius = (*mesh).get_radius();
        }
    }

    pub fn divorce_mesh(&mut self) {
        self.mesh = null_mut();
    }


    pub fn is_loaded(&self) -> bool {
        self.scene.is_some()
    }

    #[track_caller]
    pub fn load(&mut self, scene: &Scene) {
        if self.scene.is_none() && let Some(object) = self.object.take() {
            self.id = scene.add_object(&object);

            if !self.mesh.is_null() {
                unsafe {
                    (*self.mesh)._remove_unloaded_model(self.mesh_id);
                    self.mesh_id = (*self.mesh)._add_loaded_model(self);

                    if (*self.mesh).is_loaded() {
                        scene.set_object_mesh(self.id, (*self.mesh).get_ids())
                    }
                }
            }

            self.scene = Some(scene.clone());
        } else {
            panic!("Model already loaded");
        }
    }

    #[track_caller]
    pub fn unload(&mut self) {
        if let Some(mut scene) = self.scene.take() {
            self.object = Some(scene.remove_object(self.id));

            if !self.mesh.is_null() {
                unsafe {
                    (*self.mesh)._remove_loaded_model(self.mesh_id);
                    self.mesh_id = (*self.mesh)._add_unloaded_model(self);
                }
            }
        } else {
            panic!("Object is already unloaded");
        }
    }
}

impl Drop for RawModel {
    fn drop(&mut self) {
        if !self.mesh.is_null() {
            unsafe {
                if self.is_loaded() {
                    (*self.mesh)._remove_loaded_model(self.mesh_id);
                } else {
                    (*self.mesh)._remove_unloaded_model(self.mesh_id);
                }
            }
        }

        self.unload();
    }
}