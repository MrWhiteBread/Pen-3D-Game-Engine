use std::cell::RefCell;
use std::rc::Rc;
use glam::{vec3, Mat4, Quat, Vec3};
use crate::engine::back::types::camera::{CameraStructure};

pub struct Camera {
    camera: Rc<RefCell<CameraImpl>>
}

#[allow(dead_code)]
impl Camera {
    pub fn new(position: &[f32; 3]) -> Self {
        Self {
            camera: Rc::new(RefCell::new(CameraImpl::new(*position))),
        }
    }

    pub fn get_pos(&self) -> [f32; 3] {
        let camera = self.camera.borrow();
        camera.camera_structure.position.to_array()
    }

    pub fn change_pos(&self, position: &[f32; 3], lerp: &bool) {
        let mut camera = self.camera.borrow_mut();
        camera.camera_structure.position[0] += position[0];
        camera.camera_structure.position[1] += position[1];
        camera.camera_structure.position[2] += position[2];

        if !lerp {
            camera.old_camera_structure.position[0] += position[0];
            camera.old_camera_structure.position[1] += position[1];
            camera.old_camera_structure.position[2] += position[2];
        }
    }

    pub fn set_pos(&self, position: &[f32; 3], lerp: &bool) {
        let mut camera = self.camera.borrow_mut();
        camera.camera_structure.position[0] += position[0];
        camera.camera_structure.position[1] += position[1];
        camera.camera_structure.position[2] += position[2];

        if !lerp {
            camera.old_camera_structure.position[0] += position[0];
            camera.old_camera_structure.position[1] += position[1];
            camera.old_camera_structure.position[2] += position[2];
        }
    }

    pub fn get_rotation(&self) -> Quat {
        let camera = self.camera.borrow();
        camera.camera_structure.rotation
    }

    pub fn change_rotation(&self, rot: &[f32; 2], lerp: &bool) {
        let pitch = rot[0];
        let yaw   = rot[1];

        let mut camera = self.camera.borrow_mut();
        let rotation = camera.camera_structure.rotation;

        let yaw_q = Quat::from_axis_angle(Vec3::Y, yaw);

        let right = (yaw_q * rotation) * Vec3::X;

        let pitch_q = Quat::from_axis_angle(right, pitch);


        let new_rotation = (pitch_q * yaw_q * rotation).normalize();

        camera.camera_structure.rotation = new_rotation;
        if !lerp {
            camera.old_camera_structure.rotation = new_rotation;
        }
    }

    pub fn set_rotation(&self, rot: &Quat, lerp: &bool) {
        let mut camera = self.camera.borrow_mut();

        camera.camera_structure.rotation = *rot;
        if !lerp {
            camera.old_camera_structure.rotation = *rot;
        }
    }

    pub fn update(&self) {
        let mut camera = self.camera.borrow_mut();
        camera.old_camera_structure = camera.camera_structure.clone();
    }

    pub fn get_structure(&self) -> [CameraStructure; 2] {
        let camera = self.camera.borrow();
        [camera.camera_structure, camera.old_camera_structure].clone()
    }
    
    pub fn build_view_proj(&self) -> Mat4 {
        let camera = self.camera.borrow();
        camera.camera_structure.build_view_proj()
    }
    
    pub fn clone(&self) -> Self {
        Self {
            camera: self.camera.clone(),
        }
    }
}

struct CameraImpl {
    pub camera_structure: CameraStructure,
    pub old_camera_structure: CameraStructure,
}

impl CameraImpl {
    pub fn new(position: [f32; 3]) -> Self {
        let default = CameraStructure {
            position: vec3(position[0], position[1], position[2]),
            rotation: Quat::IDENTITY,
            fov_y: 80.0,
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 1000.0,
        };
        
        Self {
            camera_structure: default,
            old_camera_structure: default,
        }
    }
}