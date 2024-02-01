use ray_tracer_lib::camera::Camera;
use ray_tracer_lib::colors::Color;
use ray_tracer_lib::core::math::{Float, PI};
use ray_tracer_lib::core::transform::{rotation_y, rotation_z, scaling, translation, view_transform};
use ray_tracer_lib::core::tuple::{point, vector};
use ray_tracer_lib::lights::point_light::PointLight;
use ray_tracer_lib::material::Material;
use ray_tracer_lib::object::{build_cylinder, build_plane, build_sphere, Object};
use ray_tracer_lib::patterns::pattern::Pattern;
use crate::scene::Scene;
use ray_tracer_lib::shapes::group::Group;
use ray_tracer_lib::world::World;

pub struct GroupScene {

}
impl GroupScene {
    fn hexagon_corner() -> Object {
        let mut corner = build_sphere();
        corner.set_transformation(&translation(0.0, 0.0, -1.0) * &scaling(0.25, 0.25, 0.25));
        corner
    }

    fn hexagon_edge() -> Object {
        let mut edge = build_cylinder(0.0, 1.0);
        let t = &translation(0.0, 0.0, -1.0);
        let ry = &rotation_y(-PI / 6.0);
        let rz = &rotation_z(-PI / 2.0);
        let s = &scaling(0.25, 1.0, 0.25);
        let transformation = t * &(ry * &(rz * s));
        edge.set_transformation(transformation);
        return edge;
    }

    fn hexagon_side(material : Material) -> Object {

        let mut side = Group::new();
        let mut corner = Self::hexagon_corner();
        corner.set_material(material);
        side.add(corner);
        let mut edge = Self::hexagon_edge();
        edge.set_material(material);
        side.add(edge);
        Object::new_group(side)
    }

    fn hexagon(material: Material) -> Object {
        let mut hex = Group::new();
        const N: i32 = 6;
        for i in 0..N {
            let mut side = Self::hexagon_side(material);
            let alpha = i as Float * (2.0 * PI / (N as Float));
            let rot_y = rotation_y(alpha);
            side.set_transformation(rot_y);
            hex.add(side);
        }
        Object::new_group(hex)
    }

}
impl Scene for GroupScene {
    fn get_world(&self) -> World {
        let mut world = World::new();
        let lights = vec!(
            PointLight::new(point(0.0, 10.5, -10.0), Color::white() ),
        );
        world.set_lights(lights);

        let mut material_floor = Material::new();
        material_floor.pattern = Pattern::checker(Color::white(), Color::black());

        let mut floor = build_plane();
        floor.set_material(material_floor.clone());
        world.objects.push(floor);

        let mut hex = Self::hexagon(Material::new());
        hex.set_transformation(translation(0.0, 1.0, -0.5));
        world.objects.push(hex);

        let mut material_hex_2 = Material::new();
        material_hex_2.color = Color::black();
        material_hex_2.reflective = 0.5;
        material_hex_2.ambient = 0.0;
        let mut hex2 = Self::hexagon(material_hex_2);
        hex2.set_transformation(translation(1.0, 2.0, 1.0));
        world.objects.push(hex2);

        let mut material_hex_3 = Material::new();
        material_hex_3.color = Color::red();
        material_hex_3.shininess = 500.0;
        let mut hex3 = Self::hexagon(material_hex_3);
        hex3.set_transformation(translation(-1.5, 1.0, 1.0));
        world.objects.push(hex3);
        world
    }

    fn get_camera(&self, h_size: usize, v_size: usize) -> Camera {
        let mut camera = Camera::new(h_size, v_size, PI / 3.0);
        camera.set_transform(view_transform(point(0.0, 3.0, -2.0),
                                            point(0.0, 0.0, 0.0),
                                            vector(0.0, 1.0, 0.0)));
        camera
    }
}
