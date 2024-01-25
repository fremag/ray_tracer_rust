#[cfg(test)]
mod tests {
    use crate::shapes::cube::Cube;
    use crate::core::math::Float;
    use crate::core::ray::ray;
    use crate::core::tuple::{point, vector};

    #[test]
    fn a_ray_intersects_a_cube_test() {
        a_ray_intersects_a_cube(5.0, 0.5, 0.0, -1.0, 0.0, 0.0, 4.0, 6.0);
        a_ray_intersects_a_cube(-5.0, 0.5, 0.0, 1.0, 0.0, 0.0, 4.0, 6.0);
        a_ray_intersects_a_cube(0.5, 5.0, 0.0, 0.0, -1.0, 0.0, 4.0, 6.0);
        a_ray_intersects_a_cube(0.5, -5.0, 0.0, 0.0, 1.0, 0.0, 4.0, 6.0);
        a_ray_intersects_a_cube(0.5, 0.0, 5.0, 0.0, 0.0, -1.0, 4.0, 6.0);
        a_ray_intersects_a_cube(0.5, 0.0, -5.0, 0.0, 0.0, 1.0, 4.0, 6.0);
        a_ray_intersects_a_cube(0.0, 0.5, 0.0, 0.0, 0.0, 1.0, -1.0, 1.0);
    }

    fn a_ray_intersects_a_cube(px: Float, py: Float, pz: Float, dx: Float, dy: Float, dz: Float, t1: Float, t2: Float) {
        let c = Cube::new();
        let r = ray(point(px, py, pz), vector(dx, dy, dz));
        let xs = c.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], t1);
        assert_eq!(xs[1], t2);
    }

    #[test]
    fn a_ray_misses_a_cube_test() {
        a_ray_misses_a_cube(-2.0, 0.0, 0.0, 0.2673, 0.5345, 0.8018);
        a_ray_misses_a_cube(0.0, -2.0, 0.0, 0.8018, 0.2673, 0.5345);
        a_ray_misses_a_cube(0.0, 0.0, -2.0, 0.5345, 0.8018, 0.2673);
        a_ray_misses_a_cube(2.0, 0.0, 2.0, 0.0, 0.0, -1.0);
        a_ray_misses_a_cube(0.0, 2.0, 2.0, 0.0, -1.0, 0.0);
        a_ray_misses_a_cube(2.0, 2.0, 0.0, -1.0, 0.0, 0.0);
    }

    fn a_ray_misses_a_cube(px: Float, py: Float, pz: Float, dx: Float, dy: Float, dz: Float) {
        let c = Cube::new();
        let r = ray(point(px, py, pz), vector(dx, dy, dz));
        let xs = c.intersect(&r);
        assert_eq!(xs.len(), 0);
    }
}