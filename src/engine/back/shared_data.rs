use wgpu::Buffer;
use crate::engine::back::types::camera::CameraStructure;

#[derive(Clone)]
pub struct SharedData {
    pub queue: wgpu::Queue,
    pub camera_structure: [CameraStructure; 2],

    pub scene_light: f32,
    pub old_scene_light: f32,
    
    pub objects_count: u32,
    pub lights_count: u32,
    
    pub staging_buffers: Vec<StagingBuffers>,
}

impl SharedData {
    pub fn clone(&self) -> Self {
        Self {
            queue: self.queue.clone(),
            camera_structure: self.camera_structure,

            scene_light: self.scene_light,
            old_scene_light: self.old_scene_light,
            
            objects_count: self.objects_count,
            lights_count: self.lights_count,
            
            staging_buffers: self.staging_buffers.clone(),
        }
    }
}

#[derive(Clone)]
pub struct StagingBuffers {
    pub vertices_staging: Buffer,
    pub indices_staging: Buffer,
    pub object_staging: Buffer,
    pub light_staging: Buffer,

    pub vertices_count: u32,
    pub indices_count: u32,
    pub object_count: u32,
    pub light_count: u32,

    pub vertices_dest_offset: u64,
    pub indices_dest_offset: u64,
    pub object_dest_offset: u64,
    pub light_dest_offset: u64,
}

impl StagingBuffers {
    pub fn clone(&self) -> Self {
        Self {
            vertices_staging: self.vertices_staging.clone(),
            indices_staging: self.indices_staging.clone(),
            object_staging: self.object_staging.clone(),
            light_staging: self.light_staging.clone(),

            vertices_count: self.vertices_count,
            indices_count: self.indices_count,
            object_count: self.object_count,
            light_count: self.light_count,
            
            vertices_dest_offset: self.vertices_dest_offset,
            indices_dest_offset: self.indices_dest_offset,
            object_dest_offset: self.object_dest_offset,
            light_dest_offset: self.light_dest_offset,
        }
    }
}