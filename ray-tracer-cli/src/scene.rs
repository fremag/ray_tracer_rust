use ray_tracer_lib::camera::Camera;
use ray_tracer_lib::colors::Color;
use ray_tracer_lib::core::math::{Float, PI};
use ray_tracer_lib::core::transform::view_transform;
use ray_tracer_lib::core::tuple::{point, vector};
use ray_tracer_lib::lights::point_light::PointLight;
use ray_tracer_lib::material::Material;
use ray_tracer_lib::object::build_plane;
use ray_tracer_lib::patterns::pattern::Pattern;
use ray_tracer_lib::world::World;

pub trait Scene {
    fn get_world(&self) -> World;
    fn get_camera(&self, h_size: usize, v_size: usize) -> Camera;

    fn init_camera(&self, h_size: usize, v_size: usize
                   , from_x: Float, from_y: Float, from_z: Float
                   , look_x: Float, look_y: Float, look_z: Float) -> Camera {
        let mut camera = Camera::new(h_size, v_size, PI / 3.0);
        camera.set_transform(view_transform(point(from_x, from_y, from_z),
                                            point(look_x, look_y, look_z),
                                            vector(0.0, 1.0, 0.0)));
        camera
    }

    fn render(&self, h_size: usize, v_size: usize, file_path: &str) {
        let camera = self.get_camera(h_size, v_size);
        let world = self.get_world();
        let canvas = camera.render(&world, file_path);
        let result = canvas.save_png(file_path);
        match result {
            Ok(_) => { println!("Ok") }
            Err(error) => { println!("Error: {}", error) }
        }
    }

    fn init_world(&self, add_floor : bool) -> World {
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