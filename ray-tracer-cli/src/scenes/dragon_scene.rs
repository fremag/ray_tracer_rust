use std::env;
use std::fs::File;
use ray_tracer_lib::camera::Camera;
use ray_tracer_lib::colors::Color;
use ray_tracer_lib::core::math::PI;
use ray_tracer_lib::core::transform::{view_transform};
use ray_tracer_lib::core::tuple::{point, vector};
use ray_tracer_lib::lights::point_light::PointLight;
use ray_tracer_lib::obj_reader::ObjReader;
use ray_tracer_lib::object::Object;
use ray_tracer_lib::world::World;
use crate::scene::Scene;

pub struct DragonScene {}

impl Scene for DragonScene {
    fn get_world(&self) -> World {
        println!("{}", env::current_dir().unwrap().display());

        let file_path = r"./obj/dragon.obj";
        let file = File::open(file_path).unwrap();

        let mut obj_reader = ObjReader::new(file);
        obj_reader.read();

        let dragon = obj_reader.models;
        let dragon_model = dragon.get("Default").unwrap();
        let obj_dragon = Object::new_triangle(dragon_model.clone());
        let mut world = World::new();
        world.objects.push(obj_dragon);
        let lights = vec!(PointLight::new(point(-100.0, 100.0, -100.0), Color::new(1.0, 1.0, 1.0)));
        world.set_lights(lights);
        world
    }

    fn get_camera(&self, h_size: usize, v_size: usize) -> Camera {
        let mut camera = Camera::new(h_size, v_size, PI / 3.0);
        camera.set_transform(view_transform(point(-2.0, 6.0, -6.0),
                                            point(-1.0, 2.0, 0.0),
                                            vector(0.0, 1.0, 0.0)));
        camera
    }
}