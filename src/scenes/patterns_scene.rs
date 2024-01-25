use crate::camera::Camera;
use crate::colors::Color;
use crate::core::math::PI;
use crate::core::transform::{rotation_x, rotation_z, scaling, translation, view_transform};
use crate::core::tuple::{point, vector};
use crate::lights::point_light::PointLight;
use crate::material::Material;
use crate::object::{build_plane, build_sphere};
use crate::patterns::pattern::Pattern;
use crate::scene::Scene;
use crate::world::World;

pub struct PatternsScene {}

impl Scene for PatternsScene {
    fn get_world(&self) -> World {
        let mut material_floor = Material::new();
        material_floor.pattern = Pattern::checker(Color::white(), Color::black());

        let mut floor = build_plane();
        floor.set_material(material_floor.clone());

        let mut material_wall = Material::new();
        material_wall.pattern = Pattern::ring(Color::blue(), Color::black());

        let mut wall = build_plane();
        wall.set_transformation(&translation(0.0, 0.0, 4.0) * &rotation_x(PI / 2.0));
        wall.set_material(material_wall);

        let mut material_middle = Material::new();
        material_middle.pattern = Pattern::stripe(Color::red(), Color::green());
        material_middle.pattern.set_pattern_transform(&(&scaling(0.25, 0.25, 0.25) * &rotation_z(PI / 2.0)));

        let mut middle = build_sphere();
        middle.set_transformation(translation(-0.5, 1.0, 0.5));
        middle.set_material(material_middle);

        let mut material_right = Material::new();
        material_right.pattern = Pattern::gradient(Color::red(), Color::black());
        material_right.pattern.set_pattern_transform(&(&scaling(0.5, 0.5, 0.5) * &rotation_x(PI / 2.0)));
        let mut right = build_sphere();
        right.set_transformation(&translation(1.0, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5));
        right.set_material(material_right);

        let material_left = Material::new();

        let mut left = build_sphere();
        left.set_transformation(&translation(-2.0, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33));
        left.set_material(material_left);

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