use std::sync::Arc;
use wgpu::Buffer;
use crate::engine::back::types::camera::CameraStructure;

#[derive(Clone)]
pub struct StagingBuffers {
    pub queue: Arc<wgpu::Queue>,
    pub camera_structure: [CameraStructure; 2],
    
    pub vertices_staging: Buffer,
    pub indices_staging: Buffer,
    pub object_staging: Buffer,
    pub light_staging: Buffer,
    pub bone_staging: Buffer,
    pub bone_i_staging: Buffer,

    pub vertices_count: u32,
    pub indices_count: u32,
    pub object_count: u32,
    pub light_count: u32,
    pub bone_count: u32,
    pub bone_i_count: u32,

    pub scene_light: f32,
    pub old_scene_light: f32,

    pub vertices_dest_offset: u64,
    pub indices_dest_offset: u64,
    pub object_dest_offset: u64,
    pub light_dest_offset: u64,
}

impl StagingBuffers {
    pub(crate) fn clone(&self) -> StagingBuffers {
        Self {
            queue: self.queue.clone(),
            camera_structure: self.camera_structure.clone(),
            
            vertices_staging: self.vertices_staging.clone(),
            indices_staging: self.indices_staging.clone(),
            object_staging: self.object_staging.clone(),
            light_staging: self.light_staging.clone(),
            bone_staging: self.bone_staging.clone(),
            bone_i_staging: self.bone_i_staging.clone(),

            vertices_count: self.vertices_count.clone(),
            indices_count: self.indices_count.clone(),
            object_count: self.object_count.clone(),
            light_count: self.light_count.clone(),
            bone_count: self.bone_count.clone(),
            bone_i_count: self.bone_i_count.clone(),

            scene_light: self.scene_light.clone(),
            old_scene_light: self.old_scene_light.clone(),

            vertices_dest_offset: self.vertices_dest_offset.clone(),
            indices_dest_offset: self.indices_dest_offset.clone(),
            object_dest_offset: self.object_dest_offset.clone(),
            light_dest_offset: self.light_dest_offset.clone(),
        }
    }
}