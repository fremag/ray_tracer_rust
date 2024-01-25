use rand::{Rng, SeedableRng};
use crate::camera::Camera;
use crate::colors::Color;
use crate::core::math::{Float, PI};
use crate::core::transform::{scaling, translation, view_transform};
use crate::core::tuple::{point, vector};
use crate::lights::point_light::PointLight;
use crate::material::Material;
use crate::object::build_cylinder;
use crate::scene::Scene;
use crate::world::World;

pub struct CylinderScene {}

impl Scene for CylinderScene {
    fn get_world(&self) -> World {
        let mut world = World::new();
        let lights = vec!(
            PointLight::new(point(0.0, 50.0, -50.0), Color::white()),
        );
        world.set_lights(lights);
        const N: i32 = 9;
        let h_n = N as Float / 2.0;
        let mut mat = Material::new();
        let mut rng = rand::rngs::StdRng::seed_from_u64(2);

        for i in 0..N {
            for j in 0..N {
                let s_y = rng.gen::<Float>() * 0.8 + 0.2;
                let mut cyl = build_cylinder(0.0, s_y);
                let t_x = (i as Float - h_n) / 2.0;
                let t_z = (j as Float - h_n) / 2.0;

                let translation = translation(t_x, 0.0, t_z);
                let scaling = scaling(0.25, s_y, 0.25);
                mat.color = Color::new(rng.gen(), rng.gen(), rng.gen());
                mat.reflective = 0.3 + 0.5 * rng.gen::<Float>();

                let matrix = &translation * &scaling;
                cyl.set_transformation(matrix);
                cyl.set_material(mat);
                world.objects.push(cyl);
            }
        }
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