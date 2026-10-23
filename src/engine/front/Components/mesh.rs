use std::collections::HashMap;
use std::ptr::null_mut;
use crate::engine::back::render::scene::Scene;
use crate::engine::back::render::selective_systems::id::Id32;
use crate::engine::back::types::vertex::Vertex;
use crate::engine::front::components::model::{Model, RawModel};

type Id = u32;
type Index = u32;

pub struct Mesh {
    mesh: *mut RawMesh,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, radius: f32) -> Self {
        Self {
            mesh: Box::into_raw(Box::new(RawMesh::new(vertices, indices, radius))),
        }
    }

    pub fn is_loaded(&self) -> bool {
        unsafe {
            (*self.mesh).is_loaded()
        }
    }

    pub fn get_raw_mesh(&self) -> *mut RawMesh {
        self.mesh
    }
    
    pub fn get_radius(&self) -> f32 {
        unsafe {
            (*self.mesh).get_radius()
        }
    }

    pub fn break_down(&mut self) -> (Vec<Vertex>, Vec<u32>) {
        unsafe {
            (*self.mesh).break_down()
        }
    }

    #[track_caller]
    pub fn load(&mut self, scene: &Scene) {
        unsafe {
            (*self.mesh).load(scene);
        }
    }

    #[track_caller]
    pub fn unload(&mut self) {
        unsafe {
            (*self.mesh).unload();
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.mesh));
        }
    }
}


pub struct RawMesh {
    vertices: Option<Vec<Vertex>>,
    indices: Option<Vec<Index>>,
    radius: f32,

    scene: Option<Scene>,
    loaded_models: HashMap<Id, *mut RawModel>,
    unloaded_models: HashMap<Id, *mut RawModel>,

    model_ids: Id32,
    ids: [Id; 2],
}

impl RawMesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, radius: f32) -> Self {
        Self {
            vertices: Some(vertices),
            indices: Some(indices),
            radius,

            scene: None,
            loaded_models: HashMap::new(),
            unloaded_models: HashMap::new(),

            model_ids: Id32::new(),
            ids: [0; 2],
        }
    }

    pub fn get_ids(&self) -> &[Id; 2] {
        &self.ids
    }

    pub fn is_loaded(&self) -> bool {
        self.scene.is_some()
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn break_down(&mut self) -> (Vec<Vertex>, Vec<u32>) {
        (
            self.vertices.take().expect("empty vertices"),
            self.indices.take().expect("empty indices")
        )
    }

    pub fn _add_loaded_model(&mut self, model: *mut RawModel) -> Id{
        let id = self.model_ids.get_id();
        self.loaded_models.insert(id, model);
        id
    }

    pub fn _remove_loaded_model(&mut self, id: Id) {
        self.model_ids.remove(id);
        self.loaded_models.remove(&id);
    }

    pub fn _add_unloaded_model(&mut self, model: *mut RawModel) -> Id{
        let id = self.model_ids.get_id();
        self.unloaded_models.insert(id, model);
        id
    }

    pub fn _remove_unloaded_model(&mut self, id: Id) {
        self.model_ids.remove(id);
        self.unloaded_models.remove(&id);
    }

    #[track_caller]
    pub fn load(&mut self, scene: &Scene) {
        if self.scene.is_none() {
            let vertices = self.vertices.take().expect("empty vertices");
            let indices = self.indices.take().expect("empty indices");

            self.ids = scene.add_mesh(
                vertices,
                indices
            );

            unsafe {
                for model in self.loaded_models.values() {
                    scene.set_object_mesh((**model).get_id(), &self.ids);
                }
            }

            self.scene = Some(scene.clone());
        } else {
            panic!("Mesh already loaded");
        }
    }

    #[track_caller]
    pub fn unload(&mut self) {
        if let Some(mut scene) = self.scene.take() {
            let (vertices, indices) = scene.remove_mesh(&self.ids);

            self.vertices = Some(vertices);
            self.indices = Some(indices);

            unsafe {
                for model in self.loaded_models.values() {
                    (**model).divorce_mesh();
                    scene.remove_object_mesh((**model).get_id());
                }
            }
        } else {
            panic!("Mesh is already unloaded");
        }
    }
}

impl Drop for RawMesh {
    fn drop(&mut self) {
        unsafe {
            for model in self.loaded_models.values() {
                (**model).divorce_mesh();
            }

            for model in self.unloaded_models.values() {
                (**model).divorce_mesh();
            }
        }

        if self.vertices.is_none() {
            self.unload();
        }
    }
}