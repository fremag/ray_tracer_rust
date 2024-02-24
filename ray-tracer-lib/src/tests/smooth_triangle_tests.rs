#[cfg(test)]
mod tests {
    use crate::core::intersection::Intersection;
    use crate::core::math::equals;
    use crate::core::ray::ray;
    use crate::core::tuple::{point, vector};
    use crate::object::Object;
    use crate::shapes::smooth_triangle::SmoothTriangle;
    use crate::shapes::smooth_triangle_model::SmoothTriangleModel;

    fn get_test_triangle() -> SmoothTriangle {
        let p1 = point(0.0, 1.0, 0.0);
        let p2 = point(-1.0, 0.0, 0.0);
        let p3 = point(1.0, 0.0, 0.0);
        let n1 = vector(0.0, 1.0, 0.0);
        let n2 = vector(-1.0, 0.0, 0.0);
        let n3 = vector(1.0, 0.0, 0.0);
        let smooth_triangle = SmoothTriangle::new(p1, p2, p3, n1, n2, n3);
        return smooth_triangle;
    }

    #[test]
    fn basic_test() {
        let p1 = point(0.0, 1.0, 0.0);
        let p2 = point(-1.0, 0.0, 0.0);
        let p3 = point(1.0, 0.0, 0.0);
        let n1 = vector(0.0, 1.0, 0.0);
        let n2 = vector(-1.0, 0.0, 0.0);
        let n3 = vector(1.0, 0.0, 0.0);

        let smooth_triangle = get_test_triangle();
        assert_eq!(smooth_triangle.triangle.p1, p1);
        assert_eq!(smooth_triangle.triangle.p2, p2);
        assert_eq!(smooth_triangle.triangle.p3, p3);
        assert_eq!(smooth_triangle.n1, n1);
        assert_eq!(smooth_triangle.n2, n2);
        assert_eq!(smooth_triangle.n3, n3);
    }

    #[test]
    fn an_intersection_with_a_smooth_triangle_stores_u_v_test() {
        let smooth_triangle = get_test_triangle();

        let r = ray(point(-0.2, 0.3, -2.0), vector(0.0, 0.0, 1.0));
        let (_t, u, v) = smooth_triangle.intersect_uv(&r);
        assert!(equals(u, 0.45));
        assert!(equals(v, 0.25));
    }

    #[test]
    fn a_smooth_triangle_uses_u_v_to_interpolate_the_normal_test() {
        let smooth_triangle = get_test_triangle();
        let model = SmoothTriangleModel::new(vec![smooth_triangle.clone()]);
        let obj = Object::new_smooth_triangle_group(model);
        let i = Intersection::new_uv(1.0, obj, 0.45, 0.25);
        let n = smooth_triangle.normal_at(&point(0.0, 0.0, 0.0), &i);
        assert_eq!(n, vector(-0.5547, 0.83205, 0.0));
    }
}