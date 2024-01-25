#[cfg(test)]
mod tests {
    use crate::shapes::cylinder::Cylinder;
    use crate::core::math::{equals, Float, INFINITY};
    use crate::core::ray::ray;
    use crate::core::tuple::{point, Tuple, vector};

    #[test]
    fn a_ray_misses_a_cylinder_test() {
        a_ray_misses_a_cylinder(1.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        a_ray_misses_a_cylinder(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        a_ray_misses_a_cylinder(0.0, 0.0, -5.0, 1.0, 1.0, 1.0);
    }

    fn a_ray_misses_a_cylinder(x: Float, y: Float, z: Float, dir_x: Float, dir_y: Float, dir_z: Float) {
        let cyl = Cylinder::new();
        let direction = vector(dir_x, dir_y, dir_z).normalize();
        let r = ray(point(x, y, z), direction);
        let xs = cyl.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_strikes_a_cylinder_test() {
        a_ray_strikes_a_cylinder(1.0, 0.0, -5.0, 0.0, 0.0, 1.0, 5.0, 5.0);
        a_ray_strikes_a_cylinder(0.0, 0.0, -5.0, 0.0, 0.0, 1.0, 4.0, 6.0);
        a_ray_strikes_a_cylinder(0.5, 0.0, -5.0, 0.1, 1.0, 1.0, 6.80798, 7.08872);
    }

    fn a_ray_strikes_a_cylinder(x: Float, y: Float, z: Float, dir_x: Float, dir_y: Float, dir_z: Float, t0: Float, t1: Float) {
        let cyl = Cylinder::new();
        let direction = vector(dir_x, dir_y, dir_z).normalize();
        let r = ray(point(x, y, z), direction);
        let xs = cyl.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert!(equals(xs[0], t0));
        assert!(equals(xs[1], t1));
    }

    #[test]
    fn normal_vector_on_a_cylinder_test() {
        let cyl = Cylinder::new();
        assert_eq!(cyl.normal_at(&point(1.0, 0.0, 0.0)), vector(1.0, 0.0, 0.0));
        assert_eq!(cyl.normal_at(&point(0.0, 5.0, -1.0)), vector(0.0, 0.0, -1.0));
        assert_eq!(cyl.normal_at(&point(0.0, -2.0, 1.0)), vector(0.0, 0.0, 1.0));
        assert_eq!(cyl.normal_at(&point(-1.0, 1.0, 0.0)), vector(-1.0, 0.0, 0.0));
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder_test() {
        let cyl = Cylinder::new();
        assert_eq!(cyl.min, -INFINITY);
        assert_eq!(cyl.max, INFINITY);
    }


    #[test]
    fn intersecting_a_constrained_cylinder_test() {
        intersecting_a_constrained_cylinder(0.0, 1.5, 0.0, 0.1, 1.0, 0.0, 0);
        intersecting_a_constrained_cylinder(0.0, 3.0, -5.0, 0.0, 0.0, 1.0, 0);
        intersecting_a_constrained_cylinder(0.0, 0.0, -5.0, 0.0, 0.0, 1.0, 0);
        intersecting_a_constrained_cylinder(0.0, 2.0, -5.0, 0.0, 0.0, 1.0, 0);
        intersecting_a_constrained_cylinder(0.0, 1.0, -5.0, 0.0, 0.0, 1.0, 0);
        intersecting_a_constrained_cylinder(0.0, 1.5, -2.0, 0.0, 0.0, 1.0, 2);
    }

    fn intersecting_a_constrained_cylinder(x: Float, y: Float, z: Float, dir_x: Float, dir_y: Float, dir_z: Float, count: usize) {
        let cyl = Cylinder::from(1.0, 2.0, false);
        let direction = vector(dir_x, dir_y, dir_z).normalize();
        let r = ray(point(x, y, z), direction);
        let xs = cyl.intersect(&r);
        assert_eq!(xs.len(), count);
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder_test() {
        let cyl = Cylinder::new();
        assert_eq!(cyl.closed, false);
    }

    fn intersecting_the_caps_of_a_closed_cylinder(origin: &Tuple, direction: &Tuple) {
        let cyl = Cylinder::from(1.0, 2.0, true);
        let dir = direction.normalize();
        let r = ray(*origin, dir);
        let xs = cyl.intersect(&r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder_test() {
        intersecting_the_caps_of_a_closed_cylinder(&point(0.0, 3.0, 0.0), &vector(0.0, -1.0, 0.0));
        intersecting_the_caps_of_a_closed_cylinder(&point(0.0, 3.0, -2.0), &vector(0.0, -1.0, 2.0));
        intersecting_the_caps_of_a_closed_cylinder(&point(0.0, 4.0, -2.0), &vector(0.0, -1.0, 1.0));
        intersecting_the_caps_of_a_closed_cylinder(&point(0.0, 0.0, -2.0), &vector(0.0, 1.0, 2.0));
        intersecting_the_caps_of_a_closed_cylinder(&point(0.0, -1.0, -2.0), &vector(0.0, 1.0, 1.0));
    }

    #[test]
    fn the_normal_vector_on_a_cylinder_s_end_caps_test() {
        let cyl = Cylinder::from(1.0, 2.0, true);

        assert_eq!(cyl.normal_at(&point(0.0, 1.0, 0.0)), vector(0.0, -1.0, 0.0));
        assert_eq!(cyl.normal_at(&point(0.5, 1.0, 0.0)), vector(0.0, -1.0, 0.0));
        assert_eq!(cyl.normal_at(&point(0.0, 1.0, 0.5)), vector(0.0, -1.0, 0.0));
        assert_eq!(cyl.normal_at(&point(0.0, 2.0, 0.0)), vector(0.0, 1.0, 0.0));
        assert_eq!(cyl.normal_at(&point(0.5, 2.0, 0.0)), vector(0.0, 1.0, 0.0));
        assert_eq!(cyl.normal_at(&point(0.0, 2.0, 0.5)), vector(0.0, 1.0, 0.0));
    }
}