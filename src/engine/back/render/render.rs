use crate::engine::back::render::scene::{Scene};
use crate::engine::back::staging_buffers::StagingBuffers;

pub struct Render {
    scene: Option<Scene>,
    staging_buffers_example: StagingBuffers,
    set_statics: bool,
}

impl Render {
    pub fn new(staging_buffers_example: StagingBuffers) -> Self {
        Self {
            scene: None,
            staging_buffers_example,
            set_statics: false,
        }
    }

    pub fn create_scene(&mut self, chunks_size: &f32) -> Scene {
        Scene::new(self.staging_buffers_example.clone(), chunks_size)
    }

    pub fn update_camera(&mut self) {
        if self.scene.is_some() {
            self.scene.as_ref().expect("No scene present").update_camera();
        }
    }

    pub fn set_scene(&mut self, mut scene: Scene) {
        scene.set_dest_offset([0; 4]);

        self.scene = Some(scene);
        self.set_statics = true;
    }

    pub fn get_staging_buffers(&self) -> Option<StagingBuffers> {
        if self.scene.is_some() {
            return Some(self.scene.as_ref().expect("No scene present").get_staging_buffers())
        }
        None
    }

    pub fn setup_staging_buffers(&mut self) {
        if self.scene.is_some() {
            self.scene.as_mut().expect("No scene present").update_staging_buffers();
        }
    }

    pub fn setup_offsets(&mut self) {
        if self.scene.is_some() {
            self.scene.as_mut().expect("no scene").setup_offsets()
        }
    }
}