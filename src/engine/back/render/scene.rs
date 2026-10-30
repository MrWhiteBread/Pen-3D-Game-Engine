use std::cell::{RefCell};
use std::rc::Rc;
use crate::engine::back::render::selective_systems::dynamic_chunks::{DynamicChunks, Usage};
use crate::engine::back::shared_data::{SharedData, StagingBuffers};
use crate::engine::back::types::light::raw_light::LightType;
use crate::engine::back::types::object::raw_object::Object;
use crate::engine::back::types::vertex::Vertex;
use crate::engine::front::components::camera::Camera;

pub struct SceneImpl {
    pub selective_system: DynamicChunks,

    pub static_vertices: Vec<Vertex>,
    pub static_indices: Vec<u32>,
    pub static_objects: Vec<Object>,
    pub static_lights: Vec<LightType>,

    pub default_god: u32,
    pub default_god_values: [f32; 4],

    pub scene_light: f32,
    pub old_scene_light: f32,
    
    pub loaded: bool,
    pub camera: Camera,
}

impl SceneImpl {
    pub fn update_staging_buffers(&mut self, shared_data: &mut SharedData) {
        let camera = self.camera.get_structure();
        
        shared_data.camera_structure = [
            camera[0],
            camera[1],
        ];

        shared_data.scene_light = self.scene_light;
        shared_data.old_scene_light = self.old_scene_light;
        self.old_scene_light = self.scene_light;
        
        self.selective_system.set_staging(shared_data);
    }
}

#[repr(transparent)]
pub struct  Scene {
    scene: Rc<RefCell<SceneImpl>>,
}

impl Scene {
    pub(crate) fn clone(&self) -> Scene {
        Scene {
            scene: self.scene.clone(),
        }
    }
}

#[allow(dead_code)]
impl Scene {
    pub fn new(chunks_size: u32) -> Self {
        let mut selective_system  = DynamicChunks::new(chunks_size as f32);
        
        let default_god_values = [0.0, 0.0, 0.0, 0.0];
        let default_god = selective_system.add_god(&default_god_values);
        
        Self {
            scene: Rc::new(RefCell::new(SceneImpl {
                selective_system,

                static_vertices: Vec::new(),
                static_indices: Vec::new(),
                static_objects: Vec::new(),
                static_lights: Vec::new(),

                default_god,
                default_god_values,
                
                scene_light: 1.0,
                old_scene_light: 1.0,
                
                loaded: false,
                camera: Camera::new(&[0.0, 0.0, 0.0]),
            }))
        }
    }
    
    pub(super) fn loaded(&self, loaded: bool) {
        self.scene.borrow_mut().loaded = loaded;
    }
    
    pub fn update_camera(&self) { 
        self.scene.borrow().camera.update();
    }

    pub fn get_usage(&self) -> Usage {
        self.scene.borrow().selective_system.get_usage()
    }

    pub fn set_scene_light(&self, scene_light: f32, lerp: bool) {
        self.scene.borrow_mut().scene_light = scene_light;

        if !lerp {
            self.scene.borrow_mut().old_scene_light = scene_light;
        }
    }
    
    pub fn object_changed(&self, id: u32)
    {
        self.scene.borrow_mut().selective_system.object_changed(id);
    }

    pub unsafe fn get_object(&self, id: u32) -> &Object {
        unsafe {
            self.scene.borrow().selective_system.get_object(id).as_ref().expect("no object found")
        }
    }

    pub unsafe fn get_mut_object(&self, id: u32) -> &mut Object {
        unsafe {
            self.scene.borrow_mut().selective_system.get_mut_object(id).as_mut().expect("no object found")
        }
    }


    pub fn light_changed(&self, id: u32)
    {
        self.scene.borrow_mut().selective_system.light_changed(id);
    }

    pub unsafe fn get_light(&self, id: u32) -> &LightType {
        unsafe {
            self.scene.borrow().selective_system.get_light(id).as_ref().expect("no light found")
        }
    }

    pub unsafe fn get_mut_light(&self, id: u32) -> &mut LightType {
        unsafe {
            self.scene.borrow_mut().selective_system.get_mut_light(id).as_mut().expect("no light found")
        }
    }

    pub fn verify_model(&self, id: u32) {
        self.scene.borrow_mut().selective_system.verify_model(id);
    }

    pub fn verify_light(&self, id: u32) {
        self.scene.borrow_mut().selective_system.verify_light(id);
    }

    
    pub fn get_scene_light(&self) -> f32 {
        self.scene.borrow().scene_light
    }

    pub fn add_scene_light(&self, scene_light: f32, lerp: bool) {
        self.scene.borrow_mut().scene_light += scene_light;
        if !lerp {
            self.scene.borrow_mut().old_scene_light += scene_light;
        }
    }
    
    pub fn set_god(&self, god: u32, value: &[f32; 4]) {
        self.scene.borrow_mut().selective_system.set_god(god, value)
    }
    
    pub fn add_god(&self, god: &[f32; 4]) -> u32 {
        self.scene.borrow_mut().selective_system.add_god(god)
    }

    pub fn get_default_god(&self) -> u32 {
        self.scene.borrow().default_god
    }

    pub fn get_default_god_values(&self) -> [f32; 4] {
        self.scene.borrow().default_god_values
    }

    pub fn set_default_god(&self, id: u32) {
        let default_god = self.scene.borrow().default_god.clone();
        
        self.scene.borrow_mut().selective_system.remove_god(default_god);
        let values = self.scene.borrow_mut().selective_system.get_god(id);
        
        self.scene.borrow_mut().default_god_values = values;
        self.scene.borrow_mut().default_god = id;
    }

    pub fn set_default_god_dist(&self, dist: u32) {
        let default_god = self.scene.borrow().default_god.clone();
        self.scene.borrow_mut().selective_system.set_god_dist(default_god, dist as f32);
        
        self.scene.borrow_mut().default_god_values[3] = dist as f32;
    }

    pub fn set_default_god_pos(&self, pos: &[f32; 3]) {
        let default_god = self.scene.borrow().default_god.clone();
        self.scene.borrow_mut().selective_system.set_god_pos(default_god, pos);

        self.scene.borrow_mut().default_god_values[0] = pos[0];
        self.scene.borrow_mut().default_god_values[1] = pos[1];
        self.scene.borrow_mut().default_god_values[2] = pos[2];
    }
    
    pub fn remove_god(&self, id: u32) -> [f32; 4] {
        self.scene.borrow_mut().selective_system.remove_god(id)
    }

    pub fn clear_gods(&self) {
        self.scene.borrow_mut().selective_system.clear_gods()
    }

    pub fn set_camera(&self, camera: &Camera) {
        self.scene.borrow_mut().camera = camera.clone();
    }

    pub fn add_object(&self, object: &Object) -> u32 {
        self.scene.borrow_mut().selective_system.add_object(object)
    }

    pub fn add_mesh(&self, vertices: Vec<Vertex>, indices: Vec<u32>) -> [u32; 2 ] {
        self.scene.borrow_mut().selective_system.add_mesh(vertices, indices)
    }

    pub fn add_light(&self, light: &LightType) ->  u32 {
        self.scene.borrow_mut().selective_system.add_light(light)
    }
    
    pub fn remove_light(&self, id: u32) -> LightType {
        self.scene.borrow_mut().selective_system.remove_light(id)
    }
    
    pub fn remove_mesh(&self, mesh_ids: &[u32; 2]) -> (Vec<Vertex>, Vec<u32>) {
        self.scene.borrow_mut().selective_system.remove_mesh(mesh_ids)
    }
    
    pub fn remove_object(&self, id: u32) -> Object {
        self.scene.borrow_mut().selective_system.remove_object(id)
    }
    
    pub fn set_object_mesh(&self, id: u32, mesh_ids: &[u32; 2]) {
        self.scene.borrow_mut().selective_system.set_object_mesh(id, mesh_ids);
    }

    pub fn remove_object_mesh(&self, id: u32) {
        self.scene.borrow_mut().selective_system.remove_object_mesh(id);
    }

    pub fn update_staging_buffers(&self, shared_data: &mut SharedData) {
        let mut scene = self.scene.borrow_mut();
        scene.update_staging_buffers(shared_data);
    }

    pub fn new_chunk(&self, points: Vec<[f32; 3]>, vertices: &Vec<[f32; 3]>, indices: &Vec<u32>, protected: bool, private: bool) -> u32 {
        self.scene.borrow_mut().selective_system.new_manual_chunk(points, vertices, indices, protected, private)
    }
    
    pub fn set_object_chunk(&self, id: u32, chunk_id: u32) {
        self.scene.borrow_mut().selective_system.set_object_chunk(id, chunk_id);
    }

    pub fn set_light_chunk(&self, id: u32, chunk_id: u32) {
        self.scene.borrow_mut().selective_system.set_light_chunk(id, chunk_id);
    }
}