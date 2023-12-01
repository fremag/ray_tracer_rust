#[cfg(test)]
mod ray_tests {
    use crate::ray::ray;
    use crate::transform::{scaling, translation};
    use crate::tuple::{point, vector};

    #[test]
    fn creating_and_querying_a_ray_test() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let r = ray(origin, direction);
        assert_eq!(r.origin == origin, true);
        assert_eq!(r.direction == direction, true);
    }

    #[test]
    fn computing_a_point_from_a_distance_test() {
        let r = ray(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0) == point(2.0, 3.0, 4.0), true);
        assert_eq!(r.position(1.0) == point(3.0, 3.0, 4.0), true);
        assert_eq!(r.position(-1.0) == point(1.0, 3.0, 4.0), true);
        assert_eq!(r.position(2.5) == point(4.5, 3.0, 4.0), true);
    }

    #[test]
    fn translating_a_ray_test() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert!(r2.origin == point(4.0, 6.0, 8.0));
        assert!(r2.direction == vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray_test() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);
        assert!(r2.origin == point(2.0, 6.0, 12.0));
        assert!(r2.direction == vector(0.0, 3.0, 0.0));
    }
}