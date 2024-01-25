#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::colors::Color;
    use crate::lights::point_light::PointLight;
    use crate::core::matrix::Matrix;
    use crate::object::{build_cylinder, build_glass_sphere, build_plane, build_sphere, Object};
    use crate::core::ray::ray;
    use crate::core::transform::{rotation_y, rotation_z, scaling, translation, view_transform};
    use crate::core::tuple::{point, vector};
    use crate::shapes::group::Group;
    use crate::material::Material;
    use crate::core::math::{Float, PI};
    use crate::patterns::pattern::Pattern;
    use crate::world::World;

    #[test]
    fn creating_a_new_group_test() {
        let group = Group::new();
        let group_object = Object::new_group(group);
        assert_eq!(group_object.transformation(), &Matrix::<4>::identity());
        assert_eq!(group_object.group().unwrap().len(), 0)
    }

    #[test]
    fn adding_a_child_to_a_group_test() {
        let mut group = Group::new();
        let sphere1 = build_glass_sphere();
        let sphere2 = build_glass_sphere();
        let sphere3 = build_glass_sphere();

        group.add(sphere1);
        group.add(sphere2);
        group.add(sphere3);

        assert_eq!(group.len(), 3)
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group_test() {
        let g = Group::new();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert!(xs.intersections.is_empty());
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group_test() {
        let mut g = Group::new();
        let s1 = build_sphere();
        let mut s2 = build_sphere();
        s2.set_transformation(translation(0.0, 0.0, -3.0));
        let mut s3 = build_sphere();
        s3.set_transformation(translation(5.0, 0.0, 0.0));

        let s1_clone = s1.clone();
        let s2_clone = s2.clone();

        g.add(s1);
        g.add(s2);
        g.add(s3);

        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.intersections.len(), 4);
        assert_eq!(xs.intersections[0].t, 1.0);
        assert_eq!(xs.intersections[1].t, 3.0);
        assert_eq!(xs.intersections[2].t, 4.0);
        assert_eq!(xs.intersections[3].t, 6.0);

        assert_eq!(xs.intersections[0].object, &s2_clone);
        assert_eq!(xs.intersections[1].object, &s2_clone);
        assert_eq!(xs.intersections[2].object, &s1_clone);
        assert_eq!(xs.intersections[3].object, &s1_clone);
    }

    #[test]
    fn intersecting_a_transformed_group_test() {
        let mut s = build_sphere();
        s.set_transformation(translation(5.0, 0.0, 0.0));
        let trans = scaling(2.0, 2.0, 2.0);
        let mut group = Group::new();
        s.set_transformation(&trans * s.transformation());
        group.add(s);
        let r = ray(point(10.0, 0.0, -10.0), vector(0.0, 0.0, 1.0));
        let xs = group.intersect(&r);
        assert_eq!(xs.intersections.len(), 2);
    }

    #[test]
    fn intersecting_a_transformed_group_bis_test() {
        let mut s1 = build_sphere();
        s1.set_transformation(translation(5.0, 0.0, 0.0));
        let mut s2 = build_sphere();
        s2.set_transformation(translation(5.0, 1000.0, 0.0));
        let objects = vec![s1];
        let trans = scaling(2.0, 2.0, 2.0);
        let group = Group::from(objects, trans);

        let r = ray(point(10.0, 0.0, -10.0), vector(0.0, 0.0, 1.0));
        let xs = group.intersect(&r);
        assert_eq!(xs.intersections.len(), 2);
    }

    #[test]
    fn converting_a_point_from_world_to_object_space_test() {
        let mut s = build_sphere();
        s.set_transformation(translation(5.0, 0.0, 0.0));
        let g2 = Group::from(vec![s], scaling(2.0, 2.0, 2.0));
        let g1 = Group::from(vec![g2], rotation_y(PI / 2.0));

        let g2_ref = g1.group().unwrap().child(0);
        let s_ref = g2_ref.group().unwrap().child(0);
        let p = s_ref.world_to_object(&point(-2.0, 0.0, -10.0));
        assert_eq!(&p, &point(0.0, 0.0, -1.0));
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space_test() {
        let mut s = build_sphere();
        s.set_transformation(translation(5.0, 0.0, 0.0));
        let g2 = Group::from(vec![s], scaling(1.0, 2.0, 3.0));
        let g1 = Group::from(vec![g2], rotation_y(PI / 2.0));

        let g2_ref = g1.group().unwrap().child(0);
        let s_ref = g2_ref.group().unwrap().child(0);
        let sqrt3div3 = (3.0 as Float).sqrt() / 3.0;
        let n = s_ref.normal_to_world(&vector(sqrt3div3, sqrt3div3, sqrt3div3));
        assert_eq!(&n, &vector(0.2857, 0.4286, -0.8571));
    }

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
        let mut corner = hexagon_corner();
        corner.set_material(material);
        side.add(corner);
        let mut edge = hexagon_edge();
        edge.set_material(material);
        side.add(edge);
        Object::new_group(side)
    }

    fn hexagon(material: Material) -> Object {
        let mut hex = Group::new();
        const N: i32 = 6;
        for i in 0..N {
            let mut side = hexagon_side(material);
            let alpha = i as Float * (2.0 * PI / (N as Float));
            let rot_y = rotation_y(alpha);
            side.set_transformation(rot_y);
            hex.add(side);
        }
        Object::new_group(hex)
    }

    #[test]
    fn group_putting_it_together_test() {
        let mut world = World::new();
        let lights = vec!(
            PointLight::new(point(0.0, 10.5, -10.0), Color::white() ),
        );
        world.set_lights(lights);
        let mut camera = Camera::new(400, 400, PI / 3.0);
        camera.set_transform(view_transform(point(0.0, 3.0, -2.0),
                                            point(0.0, 0.0, 0.0),
                                            vector(0.0, 1.0, 0.0)));

        let mut material_floor = Material::new();
        material_floor.pattern = Pattern::checker(Color::white(), Color::black());

        let mut floor = build_plane();
        floor.set_material(material_floor.clone());
        world.objects.push(floor);

        let mut hex = hexagon(Material::new());
        hex.set_transformation(translation(0.0, 1.0, -0.5));
        world.objects.push(hex);

        let mut material_hex_2 = Material::new();
        material_hex_2.color = Color::black();
        material_hex_2.reflective = 0.5;
        material_hex_2.ambient = 0.0;
        let mut hex2 = hexagon(material_hex_2);
        hex2.set_transformation(translation(1.0, 2.0, 1.0));
        world.objects.push(hex2);

        let mut material_hex_3 = Material::new();
        material_hex_3.color = Color::red();
        material_hex_3.shininess = 500.0;
        let mut hex3 = hexagon(material_hex_3);
        hex3.set_transformation(translation(-1.5, 1.0, 1.0));
        world.objects.push(hex3);

        let canvas = camera.render(&world);
        let result = canvas.save("e:\\tmp\\group_scene.ppm");
        match result {
            Ok(_) => { print!("Ok") }
            Err(error) => { print!("Error: {}", error) }
        }
    }
}
