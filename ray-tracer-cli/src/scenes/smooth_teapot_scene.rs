use std::env;
use std::fs::File;
use ray_tracer_lib::camera::Camera;
use ray_tracer_lib::colors::Color;
use ray_tracer_lib::core::math::PI;
use ray_tracer_lib::core::transform::{rotation_x, rotation_y, rotation_z, view_transform};
use ray_tracer_lib::core::tuple::{point, vector};
use ray_tracer_lib::lights::point_light::PointLight;
use ray_tracer_lib::obj_reader::ObjReader;
use ray_tracer_lib::object::Object;
use ray_tracer_lib::world::World;
use crate::scene::Scene;

pub struct SmoothTeaPotScene {}

impl Scene for SmoothTeaPotScene {
    fn get_world(&self) -> World {
        println!("{}", env::current_dir().unwrap().display());
        let file_path = r"./obj/teapot.obj";
        let file = File::open(file_path).unwrap();

        let mut obj_reader = ObjReader::new(file);
        obj_reader.read();

        let teapot = obj_reader.smooth_models;
        let teapot_model= teapot.get("Teapot001").unwrap();
        let mut obj_teapot = Object::new_smooth_triangle(teapot_model.clone());
        obj_teapot.set_transformation(&rotation_y(-PI/4.0) * &(&rotation_x(-PI/2.0)*&rotation_z(PI/2.0)));
        let mut world = World::new();
        world.objects.push(obj_teapot);
        let lights = vec!(PointLight::new(point(-100.0, 100.0, -100.0), Color::new(1.0, 1.0, 1.0)));
        world.set_lights(lights);
        world
    }

    fn get_camera(&self, h_size: usize, v_size: usize) -> Camera {
        let mut camera = Camera::new(h_size, v_size, PI / 3.0);
        camera.set_transform(view_transform(point(-15.0, 15.0, -30.0),
                                            point(0.0, 1.0, 0.0),
                                            vector(0.0, 1.0, 0.0)));
        camera
    }
}