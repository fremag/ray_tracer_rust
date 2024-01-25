use crate::camera::Camera;
use crate::colors::Color;
use crate::core::math::{Float, PI};
use crate::core::transform::view_transform;
use crate::core::tuple::{point, vector};
use crate::lights::point_light::PointLight;
use crate::material::Material;
use crate::object::build_plane;
use crate::patterns::pattern::Pattern;
use crate::world::World;

pub trait Scene {
    fn get_world(&self) -> World;
    fn get_camera(&self, h_size: usize, v_size: usize) -> Camera;

    fn init_camera(h_size: usize, v_size : usize
                   , from_x : Float, from_y : Float, from_z : Float
                   , look_x : Float, look_y : Float, look_z : Float) -> Camera {
        let mut camera = Camera::new(h_size, v_size, PI / 3.0);
        camera.set_transform(view_transform(point(from_x, from_y, from_z),
                                            point(look_x, look_y, look_z),
                                            vector(0.0, 1.0, 0.0)));
        camera
    }

    fn render(&self, h_size : usize, v_size : usize, file_path : &str) {
        let camera = self.get_camera(h_size, v_size);
        let world = self.get_world();
        let canvas = camera.render(&world);
        let result = canvas.save(file_path);
        match result {
            Ok(_) => { print!("Ok") }
            Err(error) => { print!("Error: {}", error) }
        }
    }

    fn init_world(add_floor : bool) -> World {
        let mut world = World::new();
        let lights = vec!(
            PointLight::new(point(0.0, 10.5, -10.0), Color::white()),
        );
        world.set_lights(lights);

        if add_floor {
            let mut material_floor = Material::new();
            material_floor.pattern = Pattern::checker(Color::white(), Color::black());

            let mut floor = build_plane();
            floor.set_material(material_floor.clone());
            world.objects.push(floor);
        }

        world
    }
}
