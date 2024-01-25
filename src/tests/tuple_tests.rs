#[cfg(test)]
mod tests {
    use crate::core::math;
    use crate::core::math::{Float, SQRT2};
    use crate::core::tuple::{Tuple, vector, point};

    #[test]
    pub fn a_tuple_with_w_1_is_point_test() {
        let point = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0
        };

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
        assert_eq!(point.is_point(), true);
        assert_eq!(point.is_vector(), false);
    }

    #[test]
    pub fn a_tuple_with_w_zero_is_vector_test() {
        let vector = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0
        };

        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);
        assert_eq!(vector.is_point(), false);
        assert_eq!(vector.is_vector(), true);
    }

    #[test]
    pub fn vector_creates_tuples_with_w_zero_test() {
        let v = vector(4.0, -4.0, 3.0);
        assert_eq!(v.x, 4.0);
        assert_eq!(v.y, -4.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.is_vector(), true);
    }

    #[test]
    pub fn point_creates_tuples_with_w_1_test() {
        let p = point(4.0, -4.0, 3.0);
        assert_eq!(p.x, 4.0);
        assert_eq!(p.y, -4.0);
        assert_eq!(p.z, 3.0);
        assert_eq!(p.is_point(), true);
    }

    #[test]
    pub fn equals_test() {
        let p1 = point(4.0, -4.0, 3.0);
        let p2 = point(4.0, -4.0, 3.0);
        let p3 = point(4.001, -4.0, 3.0);
        let v1 = vector(4.0, -4.0, 3.0);
        assert_eq!(p1, p2);
        assert_eq!(p1 == p2, true);
        assert_eq!(p1 == p3, false);
        assert_eq!(p1 == v1, false);
    }

    #[test]
    pub fn adding_two_tuples_test() {
        let a1 = Tuple { x: 3.0, y: -2.0, z: 5.0, w: 1.0 };
        let a2 = Tuple { x: -2.0, y: 3.0, z: 1.0, w: 0.0 };
        assert_eq!(a1 + a2, Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0 })
    }

    #[test]
    pub fn subtracting_two_points_test() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let p = p1 - p2;
        assert_eq!(p, vector(-2.0, -4.0, -6.0))
    }

    #[test]
    pub fn subtracting_a_vector_from_a_point_test() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        let p = p - v;
        assert_eq!(p, point(-2.0, -4.0, -6.0))
    }

    #[test]
    pub fn subtracting_two_vectors_test() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        let v = v1 - v2;
        assert_eq!(v, vector(-2.0, -4.0, -6.0))
    }

    #[test]
    pub fn subtracting_a_vector_from_the_zero_vector_test() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        let v = zero - v;
        assert_eq!(v, vector(-1.0, 2.0, -3.0))
    }

    #[test]
    pub fn negating_a_tuple_test() {
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0))
    }

    #[test]
    pub fn multiplying_a_tuple_by_a_scalar_test() {
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0))
    }

    #[test]
    pub fn multiplying_a_tuple_by_a_fraction_test() {
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    pub fn dividing_a_tuple_by_a_scalar_test() {
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn computing_magnitude_vector_x_test() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_magnitude_vector_y_test() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_magnitude_vector_z_test() {
        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_magnitude_vector_123_test() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), (14.0 as Float).sqrt())
    }

    #[test]
    fn normalize_vector_4_0_0_gives_1_0_0_test() {
        let v = vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), vector(1.0, 0.0, 0.0))
    }

    #[test]
    fn normalize_vector_1_2_3_test() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.normalize(), vector(1.0, 2.0, 3.0) / (14.0 as Float).sqrt())
    }

    #[test]
    fn magnitude_of_a_normalized_vector_test() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(math::equals(v.normalize().magnitude(), 1.0), true)
    }

    #[test]
    fn dot_product_two_tuples_test() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        let dot_product = v1.dot(&v2);
        assert_eq!(dot_product, 20.0)
    }

    #[test]
    fn cross_product_two_vectors_test() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        let dot_product = v1 * &v2;
        assert_eq!(dot_product, vector(-1.0, 2.0, -1.0));
        let inv_dot_product = v2 * &v1;
        assert_eq!(inv_dot_product, vector(1.0, -2.0, 1.0))
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees_test() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r == vector(1.0, 1.0, 0.0), true)
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface_test() {
        let v = vector(0.0, -1.0, 0.0);
        let n = vector(SQRT2 / 2.0, SQRT2 / 2.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r == vector(1.0, 0.0, 0.0), true);
    }
}