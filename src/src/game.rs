use glam::Quat;
use crate::engine::back::render::render::Render;
use crate::engine::front::run::UpdateVar;

pub struct Game {
    render: Render,
}

impl Game {
    pub fn new(render: Render) -> Self {
        Self {
            render,
        }
    }

    pub fn update(&mut self, update_var: &mut UpdateVar) {
        
    }

    fn move_along_rotation(speed: &f32, rotation: &Quat, pos: &mut [f32; 3], direction: glam::Vec3) {
        let direction = rotation * direction;

        pos[0] -= direction.x * speed;
        pos[1] -= direction.y * speed;
        pos[2] -= direction.z * speed;
    }

    pub fn get_render(&mut self) -> &mut Render {
        &mut self.render
    }
}