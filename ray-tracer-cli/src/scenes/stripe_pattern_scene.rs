use ray_tracer_lib::camera::Camera;
use ray_tracer_lib::colors::Color;
use ray_tracer_lib::core::math::PI;
use ray_tracer_lib::core::transform::{rotation_x, scaling, translation, view_transform};
use ray_tracer_lib::core::tuple::{point, vector};
use ray_tracer_lib::lights::point_light::PointLight;
use ray_tracer_lib::material::Material;
use ray_tracer_lib::object::{build_plane, build_sphere};
use ray_tracer_lib::patterns::pattern::Pattern;
use crate::scene::Scene;
use ray_tracer_lib::world::World;

pub struct StripePatternScene {}

impl Scene for StripePatternScene {
    fn get_world(&self) -> World {
        let mut material = Material::new();
        material.color = Color::new(1.0, 0.5, 0.5);
        material.specular = 0.0;
        material.pattern = Pattern::stripe(Color::white(), material.color);

        let mut floor = build_plane();
        floor.set_material(material.clone());

        let mut wall = build_plane();
        wall.set_transformation(&translation(0.0, 0.0, 4.0) * &rotation_x(PI / 2.0));
        wall.set_material(material.clone());

        let mut middle = build_sphere();
        middle.set_transformation(translation(-0.5, 1.0, 0.5));
        middle.set_material(material);

        let mut right = build_sphere();
        right.set_transformation(&translation(1.0, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5));
        right.set_material(material);

        let mut left = build_sphere();
        left.set_transformation(&translation(-2.0, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33));
        left.set_material(material);

        let mut world = World::new();
        let lights = vec!(PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0)));
        world.set_lights(lights);
        world.set_objects(vec![
            floor,
            wall,
            left,
            middle,
            right,
        ]);
        world
    }

    fn get_camera(&self, h_size: usize, v_size: usize) -> Camera {
        let mut camera = Camera::new(h_size, v_size, PI / 3.0);
        camera.set_transform(view_transform(point(-2.5, 1.5, -5.0),
                                            point(0.0, 1.0, 0.0),
                                            vector(0.0, 1.0, 0.0)));
        camera
    }
}