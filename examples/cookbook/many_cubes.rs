use glam::Quat;
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::engine::back::render::render::Render;
use crate::engine::back::render::scene::Scene;
use crate::engine::back::types::object::Object;
use crate::engine::front::components::camera::Camera;
use crate::engine::front::components::light::Light;
use crate::engine::front::components::materials::{METAL};
use crate::engine::front::components::mesh::Mesh;
use crate::engine::front::components::model::Model;
use crate::engine::front::components::shapes::cuboid::{cuboid};
use crate::engine::front::run::UpdateVar;

pub struct Game {
    render: Render,
    scene: Scene,
    camera: Camera,

    meshes: Vec<Mesh>,
    models: Vec<Model>,

    light: Light,
}

impl Game {
    pub fn new(mut render: Render) -> Self {
        let mut meshes: Vec<Mesh> = Vec::new(); // create a Vec to hold Meshes so they don`t go out of scope
        let mut models: Vec<Model> = Vec::new(); // create a Vec to hold Models so they don`t go out of scope

        let camera = Camera::new(&[0.0; 3]); // create a camera with position

        let mut scene = render.create_scene(45.0); // create a scene with chunks size
        scene.set_camera(&camera); // set scene camera
        scene.set_scene_light(0.12, false); // set scene global light value, and disable lerp/interpolation for this change

        render.set_scene(&scene); // set scene to render


        let mut mesh = cuboid(&[1.0, 1.0, 1.0], &[1.0, 0.1, 0.1, 1.0], &METAL); // create a mesh with the shape of a cuboid using: size, color, material
        mesh.load(&scene); // load the mesh

        let object = Object::new( &[5.0, 0.0, 5.0], &[0.0, 0.0, 0.0], &[10.0, 1.0, 10.0]); // create an object using: center, rotation and size
        let mut model = Model::from(object, &mut mesh); // create a Model using an Object and a &mut Mesh
        model.load(&scene); // load the Model

        meshes.push(mesh); // moving mesh to a Vec so it doesn't go out of scope
        models.push(model); // moving model to a Vec so it doesn't go out of scope


        let mut mesh = cuboid(&[8.0, 2.0, 2.0], &[0.0, 0.9, 0.1, 1.0], &METAL);
        mesh.load(&scene);

        let object = Object::new( &[0.0, 10.0, 0.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]);
        let mut model = Model::from(object, &mut mesh);
        model.load(&scene);

        meshes.push(mesh);
        models.push(model);


        let mut mesh = cuboid(&[1.5, 1.5, 1.5], &[0.1, 1.0, 0.2, 1.0], &METAL);
        mesh.load(&scene);

        for i in 0..80 {
            for j in 0..80 {
                for k in 0..10 {
                    // places a model in a grid of 80 by 80 by 10, and uses the same mesh for every object

                    let object = Object::new(&[i as f32 * 2.0, k as f32 * -2.0, j as f32 * 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]);

                    let mut model = Model::new(object); // creates a model without a mesh
                    model.load(&scene);
                    model.set_mesh(&mut mesh); // gives the model a mesh
                    models.push(model);
                }
            }
        }
        meshes.push(mesh);

        // creates a light using: position, rotation, color, softness, range, and fov
        let mut light = Light::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.2, 1.0, 0.5], 1.2, 30.0, 360.0);
        light.load(&scene);

        Self {
            render,
            scene,
            camera,

            meshes,
            models,

            light,
        }
    }

    pub fn update(&mut self, update_var: &mut UpdateVar) {
        update_var.lock_cursor(); // locks the cursor if you press f11 and the window is in scope

        if update_var.is_cursor_locked() { // checks if cursor is locked
            update_var.add_camera_to_cursor(&mut self.camera, 0.8); // add camera rotation to cursor using a sensitivity
        }

        if update_var.tick_count == 60 { // runs when tick count gets to 60
            println!("FPS: {}", update_var.fps);
            println!("TPS: {}", update_var.tps);

            let usage = self.scene.get_usage();
            println!("indices len/size: {} / {}", usage.indices_len, usage.indices_size);
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
                PhysicalKey::Code(KeyCode::KeyH) => {
                    //increments the scene global light value when pressing the key 'H'

                    let difference = 1.0 - self.scene.get_scene_light();
                    let light = if difference <= scene_light_speed {
                        difference
                    } else {
                        scene_light_speed
                    };

                    self.scene.add_scene_light(light, true)
                }
                PhysicalKey::Code(KeyCode::KeyJ) => {
                    //decrements the scene global light value when pressing the key 'H'

                    let scene_light = self.scene.get_scene_light();
                    let light = if scene_light <= scene_light_speed {
                        -scene_light
                    } else {
                        -scene_light_speed
                    };

                    self.scene.add_scene_light(light, true);
                }

                PhysicalKey::Code(KeyCode::Space) => { // moves the camera and light up when pressing "Space"
                    self.camera.add_pos(&[0.0, speed, 0.0], true);
                    self.light.add_pos(&[0.0, speed, 0.0], true)
                }
                PhysicalKey::Code(KeyCode::ShiftLeft) => { // moves the camera and light down when pressing "Shift"
                    self.camera.add_pos(&[0.0, -speed, 0.0], true);
                    self.light.add_pos(&[0.0, -speed, 0.0], true)
                }

                PhysicalKey::Code(KeyCode::KeyW) => { // moves "pos" in +Z along rotation when pressing the key 'W'
                    Self::move_along_rotation(&speed, &rotation, &mut pos, glam::Vec3::Z);
                }
                PhysicalKey::Code(KeyCode::KeyA) => { // moves "pos" in +X along rotation when pressing the key 'A'
                    Self::move_along_rotation(&speed, &rotation, &mut pos, glam::Vec3::X);
                }
                PhysicalKey::Code(KeyCode::KeyS) => { // moves "pos" in -Z along rotation when pressing the key 'S'
                    Self::move_along_rotation(&-speed, &rotation, &mut pos, glam::Vec3::Z);
                }
                PhysicalKey::Code(KeyCode::KeyD) => { // moves "pos" in -X along rotation when pressing the key 'D'
                    Self::move_along_rotation(&-speed, &rotation, &mut pos, glam::Vec3::X);
                }

                _ => {}
            }
        }

        self.camera.add_pos(&pos, true); // adds to the camera position pos
        self.light.add_pos(&pos, true); // adds to the light position pos

        let camera_pos = self.camera.get_pos();

        // takes camera position xyz in "god_pos"
        let god_pos = [
            camera_pos[0],
            camera_pos[1],
            camera_pos[2],
        ];

        // A god is a point in space that renders all chunks where their position is equal or closer than the god`s distance
        // A default god is just a normal one that you can access without an id

        self.scene.set_default_god_pos(&god_pos); // sets default god position to "god_pos"
        self.scene.set_default_god_dist(45.0); // sets default god distance
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