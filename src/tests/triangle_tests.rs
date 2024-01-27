#[cfg(test)]
mod tests {
    use crate::core::tuple::{point, vector};
    use crate::shapes::triangle::Triangle;

    #[test]
    fn constructing_a_triangle_test() {
        let p1 = point(0.0, 1.0, 0.0);
        let p2 = point(-1.0, 0.0, 0.0);
        let p3 = point(1.0, 0.0, 0.0);
        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.p1, point(0.0, 1.0, 0.0));
        assert_eq!(t.p1, point(-1.0, 0.0, 0.0));
        assert_eq!(t.p1, point(1.0, 0.0, 0.0));

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
}