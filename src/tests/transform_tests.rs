#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::colors::Color;
    use crate::core::math::{Float, SQRT2, PI};
    use crate::core::matrix::Matrix;
    use crate::core::transform::{rotation_x, rotation_y, rotation_z, scaling, shearing, translation, view_transform};
    use crate::core::tuple::{point, vector};

    #[test]
    fn multiplying_by_a_translation_matrix_test() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        let translated = &transform * &p;
        assert_eq!(translated == point(2.0, 1.0, 7.0), true)
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix_test() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = point(-3.0, 4.0, 5.0);
        let untranslated = &inv * &p;
        assert_eq!(untranslated == point(-8.0, 7.0, 3.0), true);
    }


    #[test]
    fn translation_does_not_affect_vectors_test() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        let result = &transform * &v;
        assert_eq!(result == v, true)
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point_test() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(&transform * &p == point(-8.0, 18.0, 32.0), true)
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector_test() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(&transform * &v == vector(-8.0, 18.0, 32.0), true);
    }


    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix_test() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(&inv * &v == vector(-2.0, 2.0, 2.0), true)
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value_test() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p == point(-2.0, 3.0, 4.0), true);
    }

    #[test]
    fn rotating_a_point_around_the_x_axis_test() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_eq!(&half_quarter * &p == point(0.0, SQRT2 / 2.0, SQRT2 / 2.0), true);
        assert_eq!(&full_quarter * &p == point(0.0, 0.0, 1.0), true);
    }

    #[test]
    fn rotating_a_point_around_the_y_axis_test() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_eq!(&half_quarter * &p == point(SQRT2 / 2.0, 0.0, SQRT2 / 2.0), true);
        assert_eq!(&full_quarter * &p == point(1.0, 0.0, 0.0), true);
    }

    #[test]
    fn rotating_a_point_around_the_z_axis_test() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_eq!(&half_quarter * &p == point(-SQRT2 / 2.0, SQRT2 / 2.0, 0.0), true);
        assert_eq!(&full_quarter * &p == point(-1.0, 0.0, 0.0), true);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y_test() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p == point(5.0, 3.0, 4.0), true);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z_test() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p == point(6.0, 3.0, 4.0), true);
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x_test() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p == point(2.0, 5.0, 4.0), true);
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z_test() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p == point(2.0, 7.0, 4.0), true);
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x_test() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p == point(2.0, 3.0, 6.0), true);
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y_test() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p == point(2.0, 3.0, 7.0), true)
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence_test() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        // apply rotation first
        let p2 = &a * &p;
        assert_eq!(p2 == point(1.0, -1.0, 0.0), true);
        // then apply scaling
        let p3 = &b * &p2;
        assert_eq!(p3 == point(5.0, -5.0, 0.0), true);
        // then apply translation
        let p4 = &c * &p3;
        assert_eq!(p4 == point(15.0, 0.0, 7.0), true);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order_test() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);

        let c = translation(10.0, 5.0, 7.0);
        let t = &c * &(&b * &a);
        let result = &t * &p;
        assert_eq!(result == point(15.0, 0.0, 7.0), true);
    }

    #[test]
    fn putting_it_together_test() {
        let white = Color::new(1.0, 1.0, 1.0);
        const N: usize = 12;
        let rot = rotation_y(2.0 * PI / N as Float);
        const L: usize = 400;
        let mut canvas = Canvas::new(L, L);
        const R: Float = (L as Float) * (3.0 / 8.0);
        let mut p = point(0.0, 0.0, R);

        for _i in 0..N {
            p = &rot * &p;
            let x = (L as Float / 2.0 + p.x) as usize;
            let y = (L as Float / 2.0 + p.z) as usize;
            canvas.write_pixel(x, y, white);
        }

        let result = canvas.save("e:\\tmp\\clock.ppm");
        match result {
            Ok(_) => { print!("Ok") }
            Err(error) => { print!("Error: {}", error) }
        }
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction_test() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn the_view_transformation_moves_the_world_test() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn an_arbitrary_view_transformation_test() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        let result = Matrix::new4(
            -0.50709, 0.50709, 0.67612, -2.36643,
            0.76772, 0.60609, 0.12122, -2.82843,
            -0.35857, 0.59761, -0.71714, 0.00000,
            0.00000, 0.00000, 0.00000, 1.00000);
        assert_eq!(t, result);
    }
}