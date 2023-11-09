use crate::matrix::{Matrix};
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
    let matrix_c = Matrix::new4(20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0, 46.0, 42.0 );
    let matrix_d = &matrix_a * &matrix_b;

    assert_eq!(matrix_d == matrix_c, true )
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
    assert_eq!(result , Tuple::new(18.0, 24.0, 33.0, 1.0) )
}

#[test]
fn multiplying_a_matrix_by_the_identity_matrix_test() {
    let matrix_4x4 = Matrix::new4(0.0, 1.0, 2.0, 4.0,
                                  1.0, 2.0, 4.0, 8.0,
                                  2.0, 4.0, 8.0, 16.0,
                                  4.0, 8.0, 16.0, 32.0);

    let identity_4x4 = Matrix::<4>::identity();
    let result_4x4 = &matrix_4x4 * &identity_4x4;
    assert_eq!(matrix_4x4 == result_4x4, true );

    let matrix_2x2 = Matrix::new([[0.0, 1.0], [2.0, 4.0]]);

    let identity_2x2 = Matrix::<2>::identity();
    let result_2x2 = &matrix_2x2 * &identity_2x2;
    assert_eq!(result_2x2 == matrix_2x2, true );
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
    let matrix_4x4 = Matrix::new([[-6.0, 1.0, 1.0, 6.0 ],[-8.0, 5.0, 8.0, 6.0],[-1.0, 0.0 ,8.0, 2.0],[-7.0, 1.0, -1.0, 1.0]]);
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