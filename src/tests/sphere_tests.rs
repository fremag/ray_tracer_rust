
#[cfg(test)]
mod sphere_tests
{
    use crate::ray::ray;
    use crate::sphere::sphere;
    use crate::tuple::{point, vector};

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points_test() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent_test() {
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere_test() {
        let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere_test() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray_test() {
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }
}
