use std::sync::Arc;
use wgpu::ComputePassDescriptor;
use crate::engine::back::shared_data::StagingBuffers;
use crate::engine::back::state::State;

impl<'a> State<'a> {
    pub fn compute_pass(&mut self, encoder: &mut wgpu::CommandEncoder, objects_count: u32, lights_count: u32) {
        self.compute_once(encoder);
        self.compute_objects(encoder, objects_count);
        self.compute_lights(encoder, lights_count);
    }

    fn compute_objects(&mut self, encoder: &mut wgpu::CommandEncoder, count: u32) {
        if count == 0 {
            return;
        }
        
        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("GPU Culling Compute Pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&self.pipelines.compute_objects.pipeline);
        compute_pass.set_bind_group(0, &self.pipelines.compute_objects.bind_group, &[]);


        let workgroup_size = 64;
        let workgroups = (count + workgroup_size - 1) / workgroup_size;

        compute_pass.dispatch_workgroups(workgroups, 1, 1);
    }

    fn compute_lights(&mut self, encoder: &mut wgpu::CommandEncoder, count: u32) {
        if count == 0 {
            return;
        }
        
        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("GPU Culling Compute Pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&self.pipelines.compute_lights.pipeline);
        compute_pass.set_bind_group(0, &self.pipelines.compute_lights.bind_group, &[]);


        let workgroup_size = 64;
        let workgroups = (count + workgroup_size - 1) / workgroup_size;

        compute_pass.dispatch_workgroups(workgroups, 1, 1);
    }

    fn compute_once(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("GPU Culling Compute Pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&self.pipelines.compute_once.pipeline);
        compute_pass.set_bind_group(0, &self.pipelines.compute_once.bind_group, &[]);

        compute_pass.dispatch_workgroups(1, 1, 1);
    }
}