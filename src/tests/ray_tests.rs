#[cfg(test)]
mod ray_tests {
    use crate::ray::ray;
    use crate::tuple::{point, vector};

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let r = ray(origin, direction);
        assert_eq!(r.origin == origin, true);
        assert_eq!(r.direction == direction, true);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = ray(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
        assert_eq!(r.position( 0.0) == point(2.0, 3.0, 4.0), true);
        assert_eq!(r.position( 1.0) == point(3.0, 3.0, 4.0), true);
        assert_eq!(r.position(-1.0) == point(1.0, 3.0, 4.0), true);
        assert_eq!(r.position(2.5) == point(4.5, 3.0, 4.0), true);
    }
}