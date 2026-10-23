use crate::engine::front::components::model::{Id, Model, RawModel};

impl Model {
    pub fn get_raw_model(&self) -> *mut RawModel {
        self.model
    }

    pub fn get_pos(&self) -> [f32; 3] {
        unsafe {
            (*self.model).get_pos()
        }
    }

    pub fn get_raw_rot(&self) -> [f32; 4] {
        unsafe {
            (*self.model).get_raw_rot()
        }
    }

    pub fn get_rot(&self) -> [f32; 3] {
        unsafe {
            (*self.model).get_rot()
        }
    }

    pub fn get_size(&self) -> [f32; 3] {
        unsafe {
            (*self.model).get_size()
        }
    }
}

impl RawModel {
    pub fn get_id(&self) -> Id {
        self.id
    }
    
    pub fn get_pos(&self) -> [f32; 3] {
        if let Some(scene) = &self.scene {
            unsafe {
                let object = scene.get_object(self.id);
                [object.center[0], object.center[1], object.center[2]]
            }
        } else {
            let object = self.object.as_ref().expect("No object found");
            [object.center[0], object.center[1], object.center[2]]
        }
    }

    pub fn get_raw_rot(&self) -> [f32; 4] {
        if let Some(scene) = &self.scene {
            unsafe {
                let object = scene.get_object(self.id);
                object.rot
            }
        } else {
            let object = self.object.as_ref().expect("No object found");
            object.rot
        }
    }

    pub fn get_rot(&self) -> [f32; 3] {
        self.angle
    }

    pub fn get_size(&self) -> [f32; 3] {
        if let Some(scene) = &self.scene {
            unsafe {
                let object = scene.get_object(self.id);
                [object.size[0], object.size[1], object.size[2]]
            }
        } else {
            let object = self.object.as_ref().expect("No object found");
            [object.size[0], object.size[1], object.size[2]]
        }
    }
}