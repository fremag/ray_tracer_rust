use std::ops::{Index, Mul};
use crate::math::{equals, Float};

pub struct Matrix<const N: usize> {
    data: [[Float; N]; N]
}

impl Matrix<2> {
    pub fn new2(
        m00: Float, m01: Float,
        m10: Float, m11: Float
    ) -> Matrix<2> {
        let data = [
            [m00, m01],
            [m10, m11]
        ];
        Matrix { data }
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
}

impl Matrix<3> {
    pub fn new3(
        m00: Float, m01: Float, m02: Float,
        m10: Float, m11: Float, m12: Float,
        m20: Float, m21: Float, m22: Float,
    ) -> Matrix<3> {
        let data = [
            [m00, m01, m02],
            [m10, m11, m12],
            [m20, m21, m22],
        ];
        Matrix { data }
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

impl<const N : usize> Mul<Matrix<N>> for Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: Matrix<N>) -> Matrix<N> {
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