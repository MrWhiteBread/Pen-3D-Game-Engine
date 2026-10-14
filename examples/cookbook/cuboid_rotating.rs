use glam::Quat;
use crate::engine::back::render::render::Render;
use crate::engine::back::render::scene::Scene;
use crate::engine::back::types::object::Object;
use crate::engine::front::components::camera::Camera;
use crate::engine::front::components::light::Light;
use crate::engine::front::components::materials::METAL;
use crate::engine::front::components::mesh::Mesh;
use crate::engine::front::components::model::Model;
use crate::engine::front::components::shapes::cuboid::cuboid;
use crate::engine::front::run::UpdateVar;

pub struct Game {
    render: Render,

    scene: Scene,
    camera: Camera,

    mesh: Mesh,
    model: Model,

    light: Light,
}

impl Game {
    pub fn new(mut render: Render) -> Self {
        let camera = Camera::new(&[0.0, 0.0, 0.0]); // creates a camera using position
        let mut scene = render.create_scene(10.0); //creates a scene using chunk size

        scene.set_camera(&camera); // sets the camera
        scene.set_default_god_dist(10.0); // sets the default "god" distance
        // (a god is a point in space that renders everything around it based on its distance)

        scene.set_scene_light(0.0, false); // sets scene light/global light to 0.0 without using interpolation/lerp
        render.set_scene(&mut scene); // sets the scene to render

        // creates a mesh with the shape of a cuboid using size
        let mut mesh = cuboid(&[2.0, 2.0, 1.0]);
        mesh.load(&scene); // loads the mesh

        // creates an object using position/center rotation, size, color and material
        let object = Object::new(&[0.0, -2.0, -5.0], &[0.0; 3], &[1.0; 3], &[1.0, 0.5, 0.3, 1.0], &METAL);
        let mut model = Model::from(object, &mut mesh); // creates a model using "object" and a &mut of "mesh"
        model.load(&scene); // loads the model

        // creates a light using position rotation color softness range and fov
        let mut light = Light::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0, 0.5], 2.2, 10.0, 360.0);
        light.load(&scene); // loads the light

        Self {
            render,

            scene,
            camera,

            mesh,
            model,

            light,
        }
    }

    pub fn update(&mut self, update_var: &mut UpdateVar) {
        // adds to the model rotation the value and turns interpolation / lerp on
        self.model.add_rot(&[0.5, 0.5, 0.5], true);
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