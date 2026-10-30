use std::mem;
use crate::engine::back::render::scene::{Scene};
use crate::engine::back::shared_data::{SharedData, StagingBuffers};

pub struct Render {
    scene: Option<Scene>,
    shared_data: SharedData,

    stage: bool,
    stage1_buffers: Vec<StagingBuffers>,
    stage2_buffers: Vec<StagingBuffers>,
}

impl Render {
    pub fn new(mut shared_data: SharedData) -> Self {
        let mut staging_buffers = mem::take(&mut shared_data.staging_buffers);
        let stage2_buffers = staging_buffers.split_off(staging_buffers.len() / 2);

        Self {
            scene: None,
            shared_data,

            stage: true,
            stage1_buffers: staging_buffers,
            stage2_buffers,
        }
    }

    pub fn update_camera(&mut self) {
        if self.scene.is_some() {
            self.scene.as_ref().expect("No scene present").update_camera();
        }
    }

    pub fn set_scene(&mut self, scene: &Scene) {
        scene.loaded(true);

        if let Some(scene) = self.scene.take() {
            scene.loaded(false);
        }

        self.scene = Some(scene.clone());
    }

    pub fn get_shared_data(&self) -> SharedData {
        self.shared_data.clone()
    }

    pub fn update_staging_buffers(&mut self) {
        if self.stage {
            self.shared_data.staging_buffers = self.stage1_buffers.clone();
        } else {
            self.shared_data.staging_buffers = self.stage2_buffers.clone();
        }

        self.stage = !self.stage;

        if self.scene.is_some() {
            self.scene.as_mut().expect("No scene present").update_staging_buffers(&mut self.shared_data);
        }
    }
}