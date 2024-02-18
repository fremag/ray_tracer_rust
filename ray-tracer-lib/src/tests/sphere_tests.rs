#[cfg(test)]
mod tests {
    use crate::core::math::{Float, PI, SQRT2};
    use crate::object::build_sphere;
    use crate::core::ray::ray;
    use crate::shapes::sphere::sphere;
    use crate::core::transform::{rotation_z, scaling, translation};
    use crate::core::tuple::{point, vector};
    use crate::tests::helper::tests::dummy_intersection;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points_test() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = build_sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent_test() {
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = build_sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere_test() {
        let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = build_sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere_test() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = build_sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray_test() {
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = build_sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis_test() {
        let s = sphere();
        let n = s.normal_at(point(1.0, 0.0, 0.0), &dummy_intersection());
        let expected_n = vector(1.0, 0.0, 0.0);
        assert_eq!(n, expected_n)
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis_test() {
        let s = sphere();
        let n = s.normal_at(point(0.0, 1.0, 0.0), &dummy_intersection());
        let expected_n = vector(0.0, 1.0, 0.0);
        assert_eq!(n, expected_n)
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis_test() {
        let s = sphere();
        let n = s.normal_at(point(0.0, 0.0, 1.0), &dummy_intersection());
        let expected_n = vector(0.0, 0.0, 1.0);
        assert_eq!(n, expected_n)
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_non_axial_point_test() {
        let s = sphere();
        let v = (3.0 as Float).sqrt() / 3.0;
        let n = s.normal_at(point(v, v, v), &dummy_intersection());
        let expected_n = vector(v, v, v);
        assert_eq!(n, expected_n)
    }

    #[test]
    fn the_normal_is_a_normalized_vector_test() {
        let s = sphere();
        let v = (3.0 as Float).sqrt() / 3.0;
        let n = s.normal_at(point(v, v, v), &dummy_intersection());
        let expected_n = vector(v, v, v).normalize();
        assert_eq!(n, expected_n)
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere_test() {
        let mut s = build_sphere();
        s.set_transformation(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(point(0.0, 1.0 + SQRT2 / 2.0, -SQRT2 / 2.0), &dummy_intersection());
        assert_eq!(n == vector(0.0, SQRT2 / 2.0, -SQRT2 / 2.0), true);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere_test() {
        let mut s = build_sphere();
        let m = &scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0);
        s.set_transformation(m);
        let n = s.normal_at(point(0.0, SQRT2 / 2.0, -SQRT2 / 2.0), &dummy_intersection());
        assert_eq!(n == vector(0.0, 0.97014, -0.24254), true)
    }
}