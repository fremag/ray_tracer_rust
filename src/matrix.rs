use std::ops::{Index, Mul};
use crate::math::{equals, Float};
use crate::tuple::Tuple;

pub trait Determinant {
    fn determinant(&self) -> Float;
    fn minor(&self, row : usize, col : usize) -> Float;
    fn cofactor(&self, row : usize, col : usize) -> Float {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            - self.minor(row, col)
        }
    }
}

#[derive(Debug)]
pub struct Matrix<const N: usize> {
    data: [[Float; N]; N]
}

impl<const N : usize> Matrix<N> {
    pub fn identity() -> Matrix<N> {
        let mut matrix_identity: Matrix<N> = Matrix { data: [[0.0; N]; N] };
        for i in 0..N {
            matrix_identity.data[i][i] = 1.0;
        }
        matrix_identity
    }

    pub fn new(data: [[Float; N]; N]) -> Matrix<N> {
        Matrix { data }
    }

    pub fn transpose(&self) -> Matrix<N> {
        let mut data  = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                data[i][j] = self.data[j][i];
            }
        }
        Matrix {data}
    }
}

impl Determinant for Matrix<2> {

    fn determinant(&self) -> Float {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }

    fn minor(&self, _row: usize, _col: usize) -> Float {
        0.0
    }

    fn cofactor(&self, _row: usize, _col: usize) -> Float {
        0.0
    }
}

impl Matrix<4> {
    pub fn new4(
        m00: Float, m01: Float, m02: Float, m03: Float,
        m10: Float, m11: Float, m12: Float, m13: Float,
        m20: Float, m21: Float, m22: Float, m23: Float,
        m30: Float, m31: Float, m32: Float, m33: Float,
    ) -> Matrix<4> {
        let data = [
            [m00, m01, m02, m03],
            [m10, m11, m12, m13],
            [m20, m21, m22, m23],
            [m30, m31, m32, m33],
        ];
        Matrix { data }
    }

    pub fn sub_matrix(&self, remove_row : usize, remove_col : usize) -> Matrix<3> {
        let sub_data = sub_mat::<4,3>(remove_row, remove_col, &(self.data));
        Matrix::<3> { data: *sub_data }
    }
}

fn sub_mat<const SIZE: usize, const SUB_SIZE: usize>(remove_row : usize, remove_col : usize, data: &[[Float; SIZE]; SIZE]) -> Box<[[Float; SUB_SIZE]; SUB_SIZE]> {
    let mut sub_data : [[Float; SUB_SIZE]; SUB_SIZE] = [[0.0; SUB_SIZE]; SUB_SIZE];

    let mut new_row = 0;
    for old_row in 0..SIZE {
        if old_row == remove_row {
            continue;
        }
        let mut new_col = 0;
        for old_col in 0..SIZE {
            if old_col == remove_col {
                continue;
            }
            sub_data[new_row][new_col] = data[old_row][old_col];
            new_col += 1;
        }
        new_row += 1;
    }

    Box::new(sub_data)
}

impl Matrix<3> {
    pub fn sub_matrix(&self, remove_row : usize, remove_col : usize) -> Matrix<2> {
        let sub_data = sub_mat::<3,2>(remove_row, remove_col, &(self.data));
        Matrix::<2> { data: *sub_data }
    }
}

impl Determinant for Matrix<3> {
    fn determinant(&self) -> Float {
        let m0 =  self.cofactor(0,0);
        let m1 =  self.cofactor(0,1);
        let m2 =  self.cofactor(0,2);
        let v0 = self.data[0][0];
        let v1 = self.data[0][1];
        let v2 = self.data[0][2];

        let det = m0 * v0 + m1 * v1 + m2 * v2;
        det
    }

    fn minor(&self, row : usize, col : usize) -> Float {
        let sub_matrix = self.sub_matrix(row, col);
        sub_matrix.determinant()
    }
}

impl Determinant for Matrix<4> {
    fn determinant(&self) -> Float {
        let m0 =  self.cofactor(0,0);
        let m1 =  self.cofactor(0,1);
        let m2 =  self.cofactor(0,2);
        let m3 =  self.cofactor(0,3);
        let v0 = self.data[0][0];
        let v1 = self.data[0][1];
        let v2 = self.data[0][2];
        let v3 = self.data[0][3];

        let det = m0 * v0 + m1 * v1 + m2 * v2 + m3 * v3;
        det
    }

    fn minor(&self, row : usize, col : usize) -> Float {
        let sub_matrix = self.sub_matrix(row, col);
        sub_matrix.determinant()
    }
}
impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [Float; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N : usize> PartialEq for Matrix<N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            for j in 0..N {
                if ! equals( self[i][j], other[i][j]) {
                    return false;
                }
            }

        }

        return true;
    }
}

impl<const N : usize> Mul<&Matrix<N>> for &Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: &Matrix<N>) -> Matrix<N> {
        let mut data = [[0.0; N]; N];
        for row in 0..N {
            for col in 0..N {
                let mut c: Float  = 0.0;
                for k in 0..N {
                        c += self[row][k] * rhs[k][col]
                }
                data[row][col] = c;
            }
        }
        let matrix : Matrix<N> = Matrix {data};
        matrix
    }
}

impl<const N : usize> Mul<&Tuple> for &Matrix<N> {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Tuple {
        let mut data = [0.0; N];
        for row in 0..N {
            let mut c: Float  = 0.0;
            for col in 0..N {
                c += self[row][col] * rhs[col];
            }
            data[row]= c;
        }
        let tuple : Tuple = Tuple {x: data[0], y: data[1], z: data[2], w: data[3]};
        tuple
    }
}
