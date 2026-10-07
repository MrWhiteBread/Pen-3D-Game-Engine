use std::sync::Arc;
use std::sync::atomic::Ordering;
use wgpu::BufferAddress;
use crate::engine::back::staging_buffers::StagingBuffers;
use crate::engine::back::state::State;
use crate::engine::back::types::camera::CameraUniform;
use crate::engine::back::types::globals::Globals;
use crate::engine::back::types::light::LightType;
use crate::engine::back::types::object::Object;
use crate::engine::back::types::vertex::Vertex;

impl<'a> State<'a> {
    pub(crate) fn update_buffers(&self, encoder: &mut wgpu::CommandEncoder, staging_buffers: &Arc<StagingBuffers>) {
        let del_time = (self.del_timer.load(Ordering::Relaxed) / self.settings.time_per_update) as f32;
        let global = Globals {
            t: del_time,
            scene_light: staging_buffers.scene_light,
            old_scene_light: staging_buffers.old_scene_light,
            lerp_scene_light: 0.0,

            screen_size: [self.settings.size.width as f32, self.settings.size.height as f32],

            padding: [0.0; 2],
        };

        self.queue.write_buffer(
            &self.buffers.globals,
            0,
            bytemuck::bytes_of(&global),
        );

        self.queue.write_buffer(
            &self.buffers.count_light_buffer,
            0,
            bytemuck::bytes_of(&0),
        );

        self.queue.write_buffer(
            &self.buffers.count_buffer,
            0,
            bytemuck::bytes_of(&0),
        );

        let camera = CameraUniform::new_from_struct(&staging_buffers.camera_structure[0].lerp(&staging_buffers.camera_structure[1], &del_time));

        self.queue.write_buffer(
            &self.buffers.camera_buffer,
            0,
            bytemuck::bytes_of(&camera),
        );

        if self.ticks_updated.load(Ordering::Relaxed) {
            self.ticks_updated.store(false, Ordering::Relaxed);

            self.queue.write_buffer(
                &self.buffers.object_count_buffer,
                0,
                bytemuck::bytes_of(&staging_buffers.object_count),
            );

            self.queue.write_buffer(
                &self.buffers.light_count_buffer,
                0,
                bytemuck::bytes_of(&staging_buffers.light_count),
            );

            encoder.copy_buffer_to_buffer(
                &staging_buffers.vertices_staging,
                0,
                &self.buffers.vertices_buffer,
                staging_buffers.vertices_dest_offset,
                (size_of::<Vertex>() * staging_buffers.vertices_count as usize) as BufferAddress,
            );

            encoder.copy_buffer_to_buffer(
                &staging_buffers.indices_staging,
                0,
                &self.buffers.indices_buffer,
                staging_buffers.indices_dest_offset,
                (size_of::<u32>() * staging_buffers.indices_count as usize) as BufferAddress,
            );

            encoder.copy_buffer_to_buffer(
                &staging_buffers.object_staging,
                0,
                &self.buffers.object_buffer,
                staging_buffers.object_dest_offset,
                (size_of::<Object>() * staging_buffers.object_count as usize) as BufferAddress,
            );

            encoder.copy_buffer_to_buffer(
                &staging_buffers.light_staging,
                0,
                &self.buffers.light_buffer,
                staging_buffers.light_dest_offset,
                (size_of::<LightType>() * staging_buffers.light_count as usize) as BufferAddress,
            );
        }
    }

}