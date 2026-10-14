mod get;
mod set;
mod add;
mod global;

use std::cell::RefCell;
use std::ptr::null_mut;
use std::rc::Rc;
use glam::{EulerRot, Quat};
use crate::engine::back::render::scene::Scene;
use crate::engine::back::types::object::Object;
use crate::engine::front::components::mesh::Mesh;

type Id = u32;

pub struct Model {
    scene: Option<Scene>,

    object: Option<Object>,
    mesh: *mut Mesh,
    angle: [f32; 3],

    mesh_id: Id,
    id: Id,
}

#[allow(dead_code)]
impl Model {
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

    pub fn from(mut object: Object, mesh: &mut Mesh) -> Self {
        object.set_mesh(mesh);

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

        _self.mesh_id = mesh._add_unloaded_model(&mut _self);

        _self
    }

    pub fn divorce_mesh(&mut self) {
        self.mesh = null_mut();
    }
    
    
    pub fn is_loaded(&self) -> bool {
        self.scene.is_some()
    }
    
    pub fn load(&mut self, scene: &Scene) {
        if self.scene.is_none() {
            self.id = scene.add_object(self.object.take().expect("No object found"));

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
        }
    }
    
    pub fn unload(&mut self) {
        //TODO: 
    }
}

impl Drop for Model {
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