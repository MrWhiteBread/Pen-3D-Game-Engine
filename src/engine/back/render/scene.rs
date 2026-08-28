use std::cell::{RefCell};
use std::rc::Rc;
use crate::engine::back::render::selective_systems::dynamic_chunks::{DynamicChunks, Usage};
use crate::engine::back::staging_buffers::StagingBuffers;
use crate::engine::back::types::light::LightType;
use crate::engine::back::types::object::Object;
use crate::engine::back::types::vertex::Vertex;
use crate::engine::front::components::camera::Camera;
use crate::engine::front::components::mesh::Mesh;

pub struct SceneImpl {
    pub staging_buffers: StagingBuffers,
    pub selective_system: DynamicChunks,

    pub static_vertices: Vec<Vertex>,
    pub static_indices: Vec<u32>,
    pub static_objects: Vec<Object>,
    pub static_lights: Vec<LightType>,

    pub default_god: u32,
    pub default_god_values: [f32; 4],

    pub scene_light: f32,
    pub old_scene_light: f32,
    pub camera: Camera,
}

impl SceneImpl {
    pub fn setup_staging_buffers(&mut self) {
        let camera = self.camera.get_structure();
        
        self.staging_buffers.camera_structure = [
            camera[0],
            camera[1],
        ];

        self.staging_buffers.scene_light = self.scene_light;
        self.staging_buffers.old_scene_light = self.old_scene_light;
        self.old_scene_light = self.scene_light;
        
        self.selective_system.set_staging(&mut self.staging_buffers);
    }
}

pub struct Scene {
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
    pub fn new(staging_buffers: StagingBuffers, chunks_size: &f32) -> Self {
        let mut selective_system  = DynamicChunks::new(&chunks_size);
        
        let default_god_values = [0.0, 0.0, 0.0, 40.0];
        let default_god = selective_system.add_god(&default_god_values);
        
        Self {
            scene: Rc::new(RefCell::new(SceneImpl {
                staging_buffers,
                selective_system,

                static_vertices: Vec::new(),
                static_indices: Vec::new(),
                static_objects: Vec::new(),
                static_lights: Vec::new(),

                default_god,
                default_god_values,
                
                scene_light: 1.0,
                old_scene_light: 1.0,
                camera: Camera::new(&[0.0, 0.0, 0.0]),
            }))
        }
    }
    
    pub fn update_camera(&self) { 
        self.scene.borrow().camera.update();
    }

    pub fn get_usage(&self) -> Usage {
        self.scene.borrow().selective_system.get_usage()
    }

    pub fn set_scene_light(&self, scene_light: &f32, lerp: &bool) {
        self.scene.borrow_mut().scene_light = *scene_light;

        if !lerp {
            self.scene.borrow_mut().old_scene_light = *scene_light;
        }
    }
    
    pub fn object_changed(&self, id: &u32)
    {
        self.scene.borrow_mut().selective_system.object_changed(id);
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    pub unsafe fn get_object(&self, id: &u32) -> &Object {
        self.scene.borrow().selective_system.get_object(id).as_ref().expect("no object found")
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    pub unsafe fn get_mut_object(&self, id: &u32) -> &mut Object {
        self.scene.borrow_mut().selective_system.get_mut_object(id).as_mut().expect("no object found")
    }


    pub fn light_changed(&self, id: &u32)
    {
        self.scene.borrow_mut().selective_system.light_changed(id);
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    pub unsafe fn get_light(&self, id: &u32) -> &LightType {
        self.scene.borrow().selective_system.get_light(id).as_ref().expect("no light found")
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    pub unsafe fn get_mut_light(&self, id: &u32) -> &mut LightType {
        self.scene.borrow_mut().selective_system.get_mut_light(id).as_mut().expect("no light found")
    }

    pub fn verify_model(&self, id: &u32) {
        self.scene.borrow_mut().selective_system.verify_model(id);
    }

    pub fn verify_light(&self, id: &u32) {
        self.scene.borrow_mut().selective_system.verify_light(id);
    }

    
    pub fn get_scene_light(&self) -> f32 {
        self.scene.borrow().scene_light
    }

    pub fn change_scene_light(&self, scene_light: &f32, lerp: &bool) {
        self.scene.borrow_mut().scene_light += *scene_light;
        if !lerp {
            self.scene.borrow_mut().old_scene_light += *scene_light;
        }
    }
    
    pub fn set_god(&mut self, god: &u32, value: &[f32; 4]) {
        self.scene.borrow_mut().selective_system.set_god(god, value)
    }
    
    pub fn add_god(&mut self, god: &[f32; 4]) -> u32 {
        self.scene.borrow_mut().selective_system.add_god(god)
    }

    pub fn get_default_god(&mut self) -> u32 {
        self.scene.borrow().default_god
    }

    pub fn get_default_god_values(&mut self) -> [f32; 4] {
        self.scene.borrow().default_god_values
    }

    pub fn set_default_god(&mut self, id: &u32) {
        let default_god = self.scene.borrow().default_god.clone();
        
        self.scene.borrow_mut().selective_system.remove_god(&default_god);
        let values = self.scene.borrow_mut().selective_system.get_god(&id);
        
        self.scene.borrow_mut().default_god_values = values;
        self.scene.borrow_mut().default_god = *id;
    }

    pub fn set_default_god_dist(&mut self, dist: &f32) {
        let default_god = self.scene.borrow().default_god.clone();
        self.scene.borrow_mut().selective_system.set_god_dist(&default_god, dist);
        
        self.scene.borrow_mut().default_god_values[3] = *dist;
    }

    pub fn set_default_god_pos(&mut self, pos: &[f32; 3]) {
        let default_god = self.scene.borrow().default_god.clone();
        self.scene.borrow_mut().selective_system.set_god_pos(&default_god, pos);

        self.scene.borrow_mut().default_god_values[0] = pos[0];
        self.scene.borrow_mut().default_god_values[1] = pos[1];
        self.scene.borrow_mut().default_god_values[2] = pos[2];
    }
    
    pub fn remove_god(&mut self, id: &u32) -> [f32; 4] {
        self.scene.borrow_mut().selective_system.remove_god(id)
    }

    pub fn clear_gods(&mut self) {
        self.scene.borrow_mut().selective_system.clear_gods()
    }

    pub fn set_camera(&mut self, camera: &Camera) {
        self.scene.borrow_mut().camera = camera.clone();
    }

    pub fn add_model(&self, object: Object, mesh: Mesh) -> u32 {
        self.scene.borrow_mut().selective_system.add_model(object, mesh)
    }

    pub fn add_light(&self, light: LightType) ->  u32 {
        self.scene.borrow_mut().selective_system.add_light(light)
    }

    pub fn update_staging_buffers(&mut self) {
        let mut scene = self.scene.borrow_mut();
        scene.setup_staging_buffers();
    }

    pub fn get_staging_buffers(&self) -> StagingBuffers {
        self.scene.borrow().staging_buffers.clone()
    }
    
    pub fn setup_offsets(&mut self) {
        let static_vertices = self.scene.borrow().static_vertices.len() as u64;
        let static_indices = self.scene.borrow().static_indices.len() as u64;
        let static_objects = self.scene.borrow().static_objects.len() as u64;
        let static_lights = self.scene.borrow().static_lights.len() as u64;

        let offset = [
            static_vertices,
            static_indices,
            static_objects,
            static_lights,
        ];

        self.set_dest_offset(offset);
    }
    
    pub fn set_dest_offset(&mut self, offset: [u64; 4]) {
        self.scene.borrow_mut().staging_buffers.vertices_dest_offset = offset[0];
        self.scene.borrow_mut().staging_buffers.indices_dest_offset = offset[1];
        self.scene.borrow_mut().staging_buffers.object_dest_offset = offset[2];
        self.scene.borrow_mut().staging_buffers.light_dest_offset = offset[3];
    }
}