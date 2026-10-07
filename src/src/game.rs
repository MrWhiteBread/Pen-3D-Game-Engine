use glam::Quat;
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey, SmolStr};
use crate::engine::back::render::render::Render;
use crate::engine::back::render::scene::Scene;
use crate::engine::back::types::object::Object;
use crate::engine::front::components::camera::Camera;
use crate::engine::front::components::light::Light;
use crate::engine::front::components::materials::METAL;
use crate::engine::front::components::model::Model;
use crate::engine::front::components::shapes::cuboid::{cuboid, lag_cuboid};
use crate::engine::front::components::shapes::randomiser::randomise;
use crate::engine::front::run::UpdateVar;

pub struct Game {
    render: Render,
    camera: Camera,
    light: Light,
    model: Model,
    scene: Scene,
}

impl Game {
    pub fn new(mut render: Render) -> Self {
        let camera = Camera::new(&[0.0, 0.0, 0.0]);
        let mut scene = render.create_scene(&16.0);
        scene.set_camera(&camera);
        scene.set_scene_light(&0.12, &false);
        render.set_scene(scene.clone());

        let object1 = Object::new( &[100.0, 0.0, 100.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]);
        let cuboid1 = lag_cuboid(&[8.0, 2.0, 2.0], &[1.0, 0.1, 0.1, 1.0], &2, &METAL);
        let mut model1 = Model::new(object1, cuboid1);
        model1.load(&scene);

        let object1 = Object::new( &[5.0, 0.0, 5.0], &[0.0, 0.0, 0.0], &[10.0, 1.0, 10.0]);
        let cuboid1 = cuboid(&[1.0, 1.0, 1.0], &[1.0, 0.1, 0.1, 1.0], &METAL);
        let mut model1 = Model::new(object1, cuboid1);
        model1.load(&scene);

        let object1 = Object::new( &[0.0, 10.0, 0.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]);
        let cuboid1 = cuboid(&[8.0, 2.0, 2.0], &[0.0, 0.9, 0.1, 1.0], &METAL);
        let mut model1 = Model::new(object1, cuboid1);
        model1.load(&scene);


        for i in 0..80 {
            for j in 0..80 {
                for k in 0..20 {
                    let object12 = Object::new(&[i as f32 * 2.0, k as f32 * -2.0, j as f32 * 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]);
                    let mut cuboid12 = cuboid(&[1.5, 1.5, 1.5], &[0.1, 1.0, 0.2, 1.0], &METAL);
                    randomise(&mut cuboid12, -0.5..0.5);

                    let mut model32 = Model::new(object12, cuboid12);
                    model32.load(&scene);
                }
            }
        }

        let mut light = Light::new([9.0, 8.0, 9.0], [0.0, 0.0, 0.0], [0.5, 0.0, 1.0, 1.0], 1.0, 21.0, 360.0);
        light.load(&scene);

        let mut light = Light::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.2, 1.0, 0.5], 1.2, 30.0, 360.0);
        light.load(&scene);

        Self {
            render,
            camera,
            light,
            model: model1,
            scene,
        }
    }

    pub fn update(&mut self, update_var: &mut UpdateVar) {
        update_var.lock_cursor();
        if update_var.is_cursor_locked() {
            update_var.add_camera_to_cursor(&mut self.camera, 0.8);
        }

        if update_var.tick_count == 60 {
            println!("FPS: {}", update_var.fps);
            println!("TPS: {}", update_var.tps);
            let usage = self.scene.get_usage();
            println!("vertices len/size: {} / {}", usage.vertices_len, usage.vertices_size);
            println!("objects len/size: {} / {}", usage.objects_len, usage.objects_size);
            println!("lights len/size: {} / {}", usage.lights_len, usage.lights_size);
            println!("\n\n");
        }

        let mut pos: [f32; 3] = [0.0, 0.0, 0.0];
        let rotation = self.camera.get_rotation();
        let speed = 0.20;

        let scene_light_speed = 0.008;

        for key in update_var.key_handler.pressed_keys.iter() {
            match key {
                PhysicalKey::Code(KeyCode::KeyH) if self.scene.get_scene_light() <= 1.0 - scene_light_speed => {
                    self.scene.change_scene_light(&scene_light_speed, &true)
                }
                PhysicalKey::Code(KeyCode::KeyJ) if self.scene.get_scene_light() >= scene_light_speed => {
                    self.scene.change_scene_light(&-scene_light_speed, &true);
                }

                PhysicalKey::Code(KeyCode::Space) => {
                    self.camera.change_pos(&[0.0, speed, 0.0], &true);
                    self.light.change_pos(&[0.0, speed, 0.0], &true)
                }
                PhysicalKey::Code(KeyCode::ShiftLeft) => {
                    self.camera.change_pos(&[0.0, -speed, 0.0], &true);
                    self.light.change_pos(&[0.0, -speed, 0.0], &true)
                }

                PhysicalKey::Code(KeyCode::KeyW) => {
                    Self::move_along_rotation(&speed, &rotation, &mut pos, glam::Vec3::Z);
                }
                PhysicalKey::Code(KeyCode::KeyA) => {
                    Self::move_along_rotation(&speed, &rotation, &mut pos, glam::Vec3::X);
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    Self::move_along_rotation(&-speed, &rotation, &mut pos, glam::Vec3::Z);
                }
                PhysicalKey::Code(KeyCode::KeyD) => {
                    Self::move_along_rotation(&-speed, &rotation, &mut pos, glam::Vec3::X);
                }

                _ => {}
            }
        }

        self.camera.change_pos(&pos, &true);
        self.light.change_pos(&pos, &true);
        self.model.change_rot(&[0.0, 1.0, 0.0], true);

        let camera_pos = self.camera.get_pos();

        let god_pos = [
            camera_pos[0],
            camera_pos[1],
            camera_pos[2],
        ];

        self.scene.set_default_god_pos(&god_pos);
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