use crate::camera::Camera;
use crate::colors::Color;
use crate::core::math::PI;
use crate::core::transform::{scaling, translation, view_transform};
use crate::core::tuple::{point, vector};
use crate::lights::point_light::PointLight;
use crate::material::Material;
use crate::object::{build_plane, build_sphere};
use crate::patterns::pattern::Pattern;
use crate::scene::Scene;
use crate::world::World;

pub struct RefractionSphereScene {}

impl Scene for RefractionSphereScene {
    fn get_world(&self) -> World {
        let mut material_floor = Material::new();
        material_floor.pattern = Pattern::checker(Color::black(), Color::white());
        material_floor.ambient = 1.0;
        material_floor.specular = 0.0;
        material_floor.reflective = 0.0;
        material_floor.diffuse = 0.0;
        material_floor.shininess = 0.0;
        material_floor.pattern.set_pattern_transform(&(&translation(0.0, 0.1, 0.0) * &scaling(2.0, 2.0, 2.0)));

        let mut floor = build_plane();
        floor.set_material(material_floor);
        floor.set_transformation(translation(0.0, -10.1, 0.0));

        let mut material_sphere = Material::new();
        material_sphere.color = Color::white();
        material_sphere.diffuse = 0.4;
        material_sphere.shininess = 300.0;
        material_sphere.specular = 0.0;
        material_sphere.reflective = 1.0;
        material_sphere.ambient = 0.0;
        material_sphere.transparency = 1.0;
        material_sphere.refractive_index = 1.5;

        let mut sphere = build_sphere();
        sphere.set_material(material_sphere);

        let mut material_bubble = Material::new();
        material_bubble.color = Color::red();
        material_bubble.diffuse = 0.4;
        material_bubble.shininess = 300.0;
        material_bubble.specular = 0.0;
        material_bubble.reflective = 1.0;
        material_bubble.ambient = 0.0;
        material_bubble.transparency = 1.0;
        material_bubble.refractive_index = 1.0;
        let mut bubble = build_sphere();
        bubble.set_transformation(scaling(0.5, 0.5, 0.5));
        bubble.set_material(material_bubble);

        let mut world = World::new();
        let lights = vec!(PointLight::new(point(20.0, 10.0, 0.0), Color::white() * 0.7));
        world.set_lights(lights);
        world.set_objects(vec![
            floor,
            sphere,
            bubble,
        ]);
        world
    }

    fn get_camera(&self, h_size: usize, v_size: usize) -> Camera {
        let mut camera = Camera::new(h_size, v_size, PI / 3.0);
        camera.set_transform(view_transform(point(0.0, 2.5, 0.0),
                                            point(0.0, 0.0, 0.0),
                                            vector(0.0, 0.0, 1.0)));
        camera
    }
}