
#[cfg(test)]
mod tests {
    use crate::shapes::cone::Cone;
    use crate::core::math::{equals, Float, SQRT2};
    use crate::core::ray::ray;
    use crate::core::tuple::{point, Tuple, vector};

    fn intersecting_a_cone_with_a_ray(origin: Tuple, direction: Tuple, t0: Float, t1: Float) {
        let shape = Cone::new();
        let direction = direction.normalize();
        let r = ray(origin, direction);
        let xs = shape.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert!(equals(xs[0], t0));
        assert!(equals(xs[1], t1));
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_test() {
        intersecting_a_cone_with_a_ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 5.0, 5.0);
        intersecting_a_cone_with_a_ray(point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0), 8.66025, 8.66025);
        intersecting_a_cone_with_a_ray(point(1.0, 1.0, -5.0), vector(-0.5, -1.0, 1.0), 4.55006, 49.44994);
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves_test() {
        let shape = Cone::new();
        let direction = vector(0.0, 1.0, 1.0).normalize();
        let r = ray(point(0.0, 0.0, -1.0), direction);
        let xs = shape.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(equals(xs[0], 0.35355));
    }

    fn intersecting_a_cone_s_end_caps(origin: Tuple, direction: Tuple, count: usize) {
        let shape = Cone::from(-0.5, 0.5, true);
        let direction = direction.normalize();
        let r = ray(origin, direction);
        let xs = shape.intersect(&r);
        assert_eq!(xs.len(), count);
    }

    #[test]
    fn intersecting_a_cone_s_end_caps_tests() {
        intersecting_a_cone_s_end_caps(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0), 0);
        intersecting_a_cone_s_end_caps(point(0.0, 0.0, -0.25), vector(0.0, 1.0, 1.0), 2);
        intersecting_a_cone_s_end_caps(point(0.0, 0.0, -0.25), vector(0.0, 1.0, 0.0), 4);
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone_test() {
        let shape = Cone::new();
        assert_eq!(shape.normal_at(&point(0.0, 0.0, 0.0)), vector(0.0, 0.0, 0.0));
        assert_eq!(shape.normal_at(&point(1.0, 1.0, 1.0)), vector(1.0, -SQRT2, 1.0));
        assert_eq!(shape.normal_at(&point(-1.0, -1.0, 0.0)), vector(-1.0, 1.0, 0.0));
    }
}