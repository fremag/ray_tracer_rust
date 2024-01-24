#[cfg(test)]
mod tests {
    use crate::math::equals;
    use crate::matrix::{Determinant, Matrix};
    use crate::tuple::Tuple;

    #[test]
    fn matrix4_test() {
        let matrix4 = Matrix::new([[1.0, 2.0, 3.0, 4.0], [5.5, 6.5, 7.5, 8.5], [9.0, 10.0, 11.0, 12.0], [13.5, 14.5, 15.5, 16.5]]);
        assert_eq!(matrix4[0][0], 1.0);
        assert_eq!(matrix4[0][3], 4.0);
        assert_eq!(matrix4[1][0], 5.5);
        assert_eq!(matrix4[1][2], 7.5);
        assert_eq!(matrix4[2][2], 11.0);
        assert_eq!(matrix4[3][0], 13.5);
        assert_eq!(matrix4[3][2], 15.5);
    }

    #[test]
    fn matrix3_test() {
        let matrix3 = Matrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        assert_eq!(matrix3[0][0], -3.0);
        assert_eq!(matrix3[1][1], -2.0);
        assert_eq!(matrix3[2][2], 1.0);
    }

    #[test]
    fn matrix2_test() {
        let matrix2 = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);
        assert_eq!(matrix2[0][0], -3.0);
        assert_eq!(matrix2[0][1], 5.0);
        assert_eq!(matrix2[1][0], 1.0);
        assert_eq!(matrix2[1][1], -2.0);
    }

    #[test]
    fn matrix_equality_with_identical_matrices_test() {
        let matrix_a = Matrix::new([[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 8.0, 7.0, 6.0], [5.0, 4.0, 3.0, 2.0]]);
        let matrix_b = Matrix::new([[1.0000001, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 8.0, 7.0, 6.0], [5.0, 4.0, 3.0, 2.0]]);

        assert_eq!(matrix_a == matrix_b, true)
    }

    #[test]
    fn matrix_equality_with_different_matrices_test() {
        let matrix_a = Matrix::new4(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0);
        let matrix_b = Matrix::new4(2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);

        assert_eq!(matrix_a == matrix_b, false)
    }

    #[test]
    fn multiplying_two_matrices_test() {
        let matrix_a = Matrix::new4(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0);
        let matrix_b = Matrix::new4(-2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0);
        let matrix_c = Matrix::new4(20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0, 46.0, 42.0);
        let matrix_d = &matrix_a * &matrix_b;

        assert_eq!(matrix_d == matrix_c, true)
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple_test() {
        let matrix = Matrix::new4
            (1.0, 2.0, 3.0, 4.0,
             2.0, 4.0, 4.0, 2.0,
             8.0, 6.0, 4.0, 1.0,
             0.0, 0.0, 0.0, 1.0);
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let result = &matrix * &tuple;
        assert_eq!(result, Tuple::new(18.0, 24.0, 33.0, 1.0))
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix_test() {
        let matrix_4x4 = Matrix::new4(0.0, 1.0, 2.0, 4.0,
                                      1.0, 2.0, 4.0, 8.0,
                                      2.0, 4.0, 8.0, 16.0,
                                      4.0, 8.0, 16.0, 32.0);

        let identity_4x4 = Matrix::<4>::identity();
        let result_4x4 = &matrix_4x4 * &identity_4x4;
        assert_eq!(matrix_4x4 == result_4x4, true);

        let matrix_2x2 = Matrix::new([[0.0, 1.0], [2.0, 4.0]]);

        let identity_2x2 = Matrix::<2>::identity();
        let result_2x2 = &matrix_2x2 * &identity_2x2;
        assert_eq!(result_2x2 == matrix_2x2, true);
    }

    #[test]
    fn transposing_a_matrix_test() {
        let matrix = Matrix::new4(0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0);
        let transposed_matrix = Matrix::new4(0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0);
        let result = matrix.transpose();

        assert_eq!(result, transposed_matrix)
    }

    #[test]
    fn transposing_identity_matrix_test() {
        let matrix = Matrix::<4>::identity();
        assert_eq!(matrix.transpose(), matrix)
    }

    #[test]
    fn sub_matrix_4x4_test() {
        let matrix_4x4 = Matrix::new([[-6.0, 1.0, 1.0, 6.0], [-8.0, 5.0, 8.0, 6.0], [-1.0, 0.0, 8.0, 2.0], [-7.0, 1.0, -1.0, 1.0]]);
        let matrix_3x3 = Matrix::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

        assert_eq!(matrix_4x4.sub_matrix(2, 1), matrix_3x3)
    }

    #[test]
    fn sub_matrix_3x3_test() {
        let matrix_3x3 = Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let matrix_2x2 = Matrix::new([[-3.0, 2.0], [0.0, 6.0]]);

        assert_eq!(matrix_3x3.sub_matrix(0, 2), matrix_2x2)
    }

    #[test]
    fn calculating_determinant_matrix_2x2_test() {
        let matrix_2x2 = Matrix::new([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(matrix_2x2.determinant(), 17.0)
    }

    #[test]
    fn calculating_minor_matrix_3x3_test() {
        let matrix_3x3 = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(matrix_3x3.minor(1, 0), 25.0)
    }

    #[test]
    fn calculating_cofactor_matrix_3x3_test() {
        let matrix_3x3 = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(matrix_3x3.minor(0, 0), -12.0);
        assert_eq!(matrix_3x3.cofactor(0, 0), -12.0);
        assert_eq!(matrix_3x3.minor(1, 0), 25.0);
        assert_eq!(matrix_3x3.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_determinant_matrix_3x3_test() {
        let matrix_3x3 = Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(matrix_3x3.cofactor(0, 0), 56.0);
        assert_eq!(matrix_3x3.cofactor(0, 1), 12.0);
        assert_eq!(matrix_3x3.cofactor(0, 2), -46.0);
        assert_eq!(matrix_3x3.determinant(), -196.0);
    }

    #[test]
    fn calculating_determinant_matrix_4x4_test() {
        let matrix_4x4 = Matrix::new([[-2.0, -8.0, 3.0, 5.0], [-3.0, 1.0, 7.0, 3.0], [1.0, 2.0, -9.0, 6.0], [-6.0, 7.0, 7.0, -9.0]]);
        assert_eq!(matrix_4x4.cofactor(0, 0), 690.0);
        assert_eq!(matrix_4x4.cofactor(0, 1), 447.0);
        assert_eq!(matrix_4x4.cofactor(0, 2), 210.0);
        assert_eq!(matrix_4x4.cofactor(0, 3), 51.0);
        assert_eq!(matrix_4x4.determinant(), -4071.0);
    }

    #[test]
    fn calculating_inverse_of_matrix_test() {
        let matrix_4x4 = Matrix::new([[-5.0, 2.0, 6.0, -8.0], [1.0, -5.0, 1.0, 8.0], [7.0, 7.0, -6.0, -7.0], [1.0, -3.0, 7.0, 4.0]]);
        assert_eq!(matrix_4x4.determinant(), 532.0);
        let inverse_matrix = matrix_4x4.inverse();

        assert_eq!(matrix_4x4.cofactor(2, 3), -160.0);
        assert_eq!(equals(inverse_matrix[3][2], -160.0 / 532.0), true);
        assert_eq!(matrix_4x4.cofactor(3, 2), 105.0);
        assert_eq!(equals(inverse_matrix[2][3], 105.0 / 532.0), true);

        let expected = Matrix::new([[0.21805, 0.45113, 0.24060, -0.04511], [-0.80827, -1.45677, -0.44361, 0.52068], [-0.07895, -0.22368, -0.05263, 0.19737], [-0.52256, -0.81391, -0.30075, 0.30639]]);
        assert_eq!(expected == inverse_matrix, true);
    }

    #[test]
    fn calculating_inverse_of_another_matrix_test() {
        let matrix_4x4 = Matrix::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0]]);
        let inverse_matrix = matrix_4x4.inverse();
        let expected = Matrix::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308]]);
        assert_eq!(expected == inverse_matrix, true);
    }

    #[test]
    fn calculating_inverse_of_a_third_matrix_test() {
        let matrix_4x4 = Matrix::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0]]);
        let inverse_matrix = matrix_4x4.inverse();
        let expected = Matrix::new([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.0333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333]]);
        assert_eq!(expected == inverse_matrix, true);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse_test() {
        let matrix_a_4x4 = Matrix::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0]]);
        let matrix_b_4x4 = Matrix::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0]]);

        let matrix_c = &matrix_a_4x4 * &matrix_b_4x4;
        let matrix_d = &matrix_c * &(matrix_b_4x4.inverse());
        assert_eq!(matrix_d == matrix_a_4x4, true);
    }
}