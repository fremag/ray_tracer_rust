#[cfg(test)]
mod tests {
    use crate::core::matrix::Matrix;
    use crate::object::{build_glass_sphere, build_sphere, Object};
    use crate::core::ray::ray;
    use crate::core::transform::{rotation_y, scaling, translation};
    use crate::core::tuple::{point, vector};
    use crate::shapes::group::Group;
    use crate::core::math::{Float, PI};

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
}
