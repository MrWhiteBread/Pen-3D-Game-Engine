use std::collections::HashMap;
use crate::engine::back::render::scene::Scene;
use crate::engine::back::render::selective_systems::id::Id32;
use crate::engine::back::types::vertex::Vertex;
use crate::engine::front::components::model::Model;

type Id = u32;
type Index = u32;

pub struct Mesh {
    vertices: Option<Vec<Vertex>>,
    indices: Option<Vec<Index>>,
    radius: f32,

    scene: Option<Scene>,
    loaded_models: HashMap<Id, *mut Model>,
    unloaded_models: HashMap<Id, *mut Model>,

    model_ids: Id32,
    ids: [Id; 2],
}

impl Mesh {
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

    pub fn _add_loaded_model(&mut self, model: *mut Model) -> Id{
        let id = self.model_ids.get_id();
        self.loaded_models.insert(id, model);
        id
    }

    pub fn _remove_loaded_model(&mut self, id: Id) {
        self.model_ids.remove(id);
        self.loaded_models.remove(&id);
    }

    pub fn _add_unloaded_model(&mut self, model: *mut Model) -> Id{
        let id = self.model_ids.get_id();
        self.unloaded_models.insert(id, model);
        id
    }

    pub fn _remove_unloaded_model(&mut self, id: Id) {
        self.model_ids.remove(id);
        self.unloaded_models.remove(&id);
    }

    pub fn load(&mut self, scene: &Scene) {
        if self.scene.is_none() {
            self.ids = scene.add_mesh(
                self.vertices.take().expect("empty vertices"),
                self.indices.take().expect("empty indices")
            );
            
            unsafe {
                for model in self.loaded_models.values() {
                    scene.set_object_mesh((**model).get_id(), &self.ids);
                }
            }

            self.scene = Some(scene.clone());
        }
    }

    pub fn unload(&mut self) {
        //TODO
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            for model in self.loaded_models.values() {
                (**model).divorce_mesh();
            }

            for model in self.unloaded_models.values() {
                (**model).divorce_mesh();
            }
        }

        self.unload();
    }
}