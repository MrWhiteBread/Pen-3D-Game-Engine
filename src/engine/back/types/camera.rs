use glam::{Mat4, Quat, Vec3};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Plane {
    pub normal: [f32; 3],
    pub d: f32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
    pub inv_view_proj: [[f32; 4]; 4],
    pub position: [f32; 4],

    pub planes: [Plane; 6],
}

impl CameraUniform {
    pub fn new_from_struct(camera: &CameraStructure ) -> Self{
        let mat = camera.build_view_proj();
        let view_proj = mat.to_cols_array_2d();

        let inv_view_proj = mat.inverse().to_cols_array_2d();
        let position = [
            camera.position.x,
            camera.position.y,
            camera.position.z,
            1.0,
        ];
        
        Self {
            view_proj,
            inv_view_proj,
            position,
            planes: [Plane {normal: [0.0; 3], d: 0.0}; 6]
        }
    }
}

#[derive(Clone, Copy)]
pub struct CameraStructure {
    pub position: Vec3,
    pub rotation: Quat,
    pub fov_y: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraStructure {
    pub fn lerp(&self, camera: &CameraStructure, t: &f32) -> CameraStructure {
        let position = camera.position.lerp(self.position, *t);
        let rotation = camera.rotation.slerp(self.rotation, *t);
        let fov_y = Self::mix(&camera.fov_y, &self.fov_y, t);
        let aspect = Self::mix(&camera.aspect, &self.aspect, t);
        let near = Self::mix(&camera.near, &self.near, t);
        let far = Self::mix(&camera.far, &self.far, t);

        CameraStructure {
            position,
            rotation,
            fov_y,
            aspect,
            near,
            far,
        }
    }

    fn mix(old: &f32, new: &f32, t: &f32) -> f32 {
        old * (1.0 - t) + new * t
    }

    pub fn build_view_proj(&self) -> Mat4 {
        let forward = self.rotation * Vec3::NEG_Z;
        let up = self.rotation * Vec3::Y;

        let view = Mat4::look_at_rh(self.position, forward + self.position, up);
        let proj = Mat4::perspective_rh(self.fov_y.to_radians(), self.aspect, self.near, self.far);

        proj * view
    }
}