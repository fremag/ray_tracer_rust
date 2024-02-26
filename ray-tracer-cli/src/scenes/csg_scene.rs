use ray_tracer_lib::camera::Camera;
use ray_tracer_lib::colors::Color;
use ray_tracer_lib::core::math::{PI};
use ray_tracer_lib::core::transform::{scaling, view_transform};
use ray_tracer_lib::core::tuple::{point, vector};
use ray_tracer_lib::lights::point_light::PointLight;
use ray_tracer_lib::material::Material;
use ray_tracer_lib::object::{build_cube, build_cylinder, build_plane, Object};
use ray_tracer_lib::patterns::pattern::Pattern;
use ray_tracer_lib::shapes::csg::{Csg, CsgOperation};
use crate::scene::Scene;
use ray_tracer_lib::world::World;

pub struct CsgScene {}

impl Scene for CsgScene {
    fn get_world(&self) -> World {
        let mut world = World::new();
        let lights = vec!(
            PointLight::new(point(0.0, 50.0, -50.0), Color::white()),
        );
        world.set_lights(lights);

        let mut material_floor = Material::new();
        material_floor.pattern = Pattern::checker(Color::white(), Color::black());

        let mut floor = build_plane();
        floor.set_material(material_floor.clone());
        world.objects.push(floor);

        let mut mat1 = Material::new();
        mat1.color = Color::red();

        let mut cyl1 = build_cylinder(0.0, 1.0);
        cyl1.set_transformation(scaling(2.0, 1.0, 2.0));
        cyl1.set_material(mat1.clone());

        let mut mat2 = Material::new();
        mat2.color = Color::green();
        let mut cube2 = build_cube();

        cube2.set_material(mat2.clone());

        let csg = Csg::new(CsgOperation::Difference, cyl1, cube2);

        world.objects.push(Object::new_csg(csg));

        world
    }

    fn get_camera(&self, h_size: usize, v_size: usize) -> Camera {
        let mut camera = Camera::new(h_size, v_size, PI / 3.0);
        camera.set_transform(view_transform(point(-0.5, 4.0, -2.0),
                                            point(0.0, 0.0, 0.0),
                                            vector(0.0, 1.0, 0.0)));
        camera
    }
}