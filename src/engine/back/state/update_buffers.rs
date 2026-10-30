use std::sync::Arc;
use std::sync::atomic::Ordering;
use arc_swap::ArcSwap;
use wgpu::BufferAddress;
use crate::engine::back::shared_data::StagingBuffers;
use crate::engine::back::state::State;
use crate::engine::back::types::camera::CameraUniform;
use crate::engine::back::types::globals::Globals;
use crate::engine::back::types::light::raw_light::LightType;
use crate::engine::back::types::object::raw_object::Object;
use crate::engine::back::types::vertex::Vertex;

impl<'a> State<'a> {
    pub(crate) fn update_buffers(&self, encoder: &mut wgpu::CommandEncoder) -> (u32, u32) {
        let shared_data = self.shared_data.load();

        let del_time = (self.del_timer.load(Ordering::Relaxed) / self.settings.time_per_update) as f32;
        let global = Globals {
            t: del_time,
            scene_light: shared_data.scene_light,
            old_scene_light: shared_data.old_scene_light,
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

        let camera = CameraUniform::new_from_struct(&shared_data.camera_structure[0].lerp(&shared_data.camera_structure[1], del_time));

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
                bytemuck::bytes_of(&shared_data.objects_count),
            );

            self.queue.write_buffer(
                &self.buffers.light_count_buffer,
                0,
                bytemuck::bytes_of(&shared_data.lights_count),
            );

            let staging_buffers = shared_data.staging_buffers.get(0).expect("staging buffer is empty??");

            if staging_buffers.vertices_count > 0 {
                encoder.copy_buffer_to_buffer(
                    &staging_buffers.vertices_staging,
                    0,
                    &self.buffers.vertices_buffer,
                    staging_buffers.vertices_dest_offset,
                    (size_of::<Vertex>() * staging_buffers.vertices_count as usize) as BufferAddress,
                );
            }

            if staging_buffers.indices_count > 0 {
                encoder.copy_buffer_to_buffer(
                    &staging_buffers.indices_staging,
                    0,
                    &self.buffers.indices_buffer,
                    staging_buffers.indices_dest_offset,
                    (size_of::<u32>() * staging_buffers.indices_count as usize) as BufferAddress,
                );
            }


            for staging_buffers in shared_data.staging_buffers.iter() {
                if staging_buffers.object_count > 0 {
                    encoder.copy_buffer_to_buffer(
                        &staging_buffers.object_staging,
                        0,
                        &self.buffers.raw_object_buffer,
                        staging_buffers.object_dest_offset,
                        (size_of::<Object>() * staging_buffers.object_count as usize) as BufferAddress,
                    );
                }

                if staging_buffers.light_count > 0 {
                    encoder.copy_buffer_to_buffer(
                        &staging_buffers.light_staging,
                        0,
                        &self.buffers.raw_light_buffer,
                        staging_buffers.light_dest_offset,
                        (size_of::<LightType>() * staging_buffers.light_count as usize) as BufferAddress,
                    );
                }
            }
        }

        (shared_data.objects_count, shared_data.lights_count)
    }

}