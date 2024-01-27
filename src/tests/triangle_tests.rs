#[cfg(test)]
mod tests {
    use crate::core::ray::ray;
    use crate::core::tuple::{point, vector};
    use crate::shapes::triangle::Triangle;

    #[test]
    fn constructing_a_triangle_test() {
        let p1 = point(0.0, 1.0, 0.0);
        let p2 = point(-1.0, 0.0, 0.0);
        let p3 = point(1.0, 0.0, 0.0);
        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.p1, point(0.0, 1.0, 0.0));
        assert_eq!(t.p2, point(-1.0, 0.0, 0.0));
        assert_eq!(t.p3, point(1.0, 0.0, 0.0));

        assert_eq!(t.e1, vector(-1.0, -1.0, 0.0));
        assert_eq!(t.e2, vector(1.0, -1.0, 0.0));
        assert_eq!(t.normal, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn finding_the_normal_on_a_triangle_test() {
        let t = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
        let n1 = t.normal_at(&point(0.0, 0.5, 0.0));
        let n2 = t.normal_at(&point(-0.5, 0.75, 0.0));
        let n3 = t.normal_at(&point(0.5, 0.25, 0.0));
        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle_test() {
        let t = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
        let r = ray(point(0.0, -1.0, -2.0), vector(0.0, 1.0, 0.0));
        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge_test() {
        let t = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
        let r = ray(point(1.0, 1.0, -2.0), vector(0.0, 0.0, 1.0));
        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge_test() {
        let t = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
        let r = ray(point(-1.0, 1.0, -2.0), vector(0.0, 0.0, 1.0));
        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge_test() {
        let t = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
        let r = ray(point(0.0, -1.0, -2.0), vector(0.0, 0.0, 1.0));
        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_strikes_a_triangle_test() {
        let t = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
        let r = ray(point(0.0, 0.5, -2.0), vector(0.0, 0.0, 1.0));
        let xs = t.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 2.0);
    }
}