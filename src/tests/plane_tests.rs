#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::colors::Color;
    use crate::lights::point_light::PointLight;
    use crate::material::Material;
    use crate::core::math::PI;
    use crate::object::{build_plane, build_sphere};
    use crate::shapes::plane::Plane;
    use crate::core::ray::ray;
    use crate::core::transform::{scaling, translation, view_transform};
    use crate::core::tuple::{point, vector};
    use crate::world::World;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere_test() {
        let p = Plane::new();
        let n1 = p.normal_at(point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane_test() {
        let p = Plane::new();
        let r = ray(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray_test() {
        let p = Plane::new();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above_test() {
        let p = build_plane();
        let r = ray(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.count(), 1);

        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, &p);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below_test() {
        let p = build_plane();
        let r = ray(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.count(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, &p);
    }

    #[test]
    fn putting_it_together_test() {
        let mut wall_material = Material::new();
        wall_material.color = Color::new(1.0, 0.9, 0.9);
        wall_material.specular = 0.0;

        let mut floor = build_plane();
        floor.set_material(wall_material.clone());

        let mut mid_material = Material::new();
        mid_material.color = Color::new(0.1, 1.0, 0.5);
        mid_material.diffuse = 0.7;
        mid_material.specular = 0.3;
        let mut middle = build_sphere();
        middle.set_transformation(translation(-0.5, 1.0, 0.5));
        middle.set_material(mid_material);

        let mut right_material = Material::new();
        right_material.color = Color::new(0.5, 1.0, 0.1);
        right_material.diffuse = 0.7;
        right_material.specular = 0.3;
        let mut right = build_sphere();
        right.set_transformation(&translation(1.5, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5));
        right.set_material(right_material);

        let mut left_material = Material::new();
        left_material.color = Color::new(1.0, 0.8, 0.1);
        left_material.diffuse = 0.7;
        left_material.specular = 0.3;

        let mut left = build_sphere();
        left.set_transformation(&translation(-1.5, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33));
        left.set_material(left_material);

        let mut world = World::new();
        let lights = vec!(PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0)));
        world.set_lights(lights);
        world.set_objects(vec![
            floor,
            left,
            middle,
            right,
        ]);

        let mut camera = Camera::new(400, 200, PI / 3.0);
        camera.set_transform(view_transform(point(0.0, 1.5, -5.0),
                                            point(0.0, 1.0, 0.0),
                                            vector(0.0, 1.0, 0.0)));

        let canvas = camera.render(&world);
        let result = canvas.save("e:\\tmp\\plane_scene.ppm");
        match result {
            Ok(_) => { print!("Ok") }
            Err(error) => { print!("Error: {}", error) }
        }
    }
}