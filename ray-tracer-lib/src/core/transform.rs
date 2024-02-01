use crate::core::math::Float;
use crate::core::matrix::Matrix;
use crate::core::tuple::Tuple;

pub fn translation(x : Float , y: Float, z : Float) -> Matrix<4> {
    Matrix::new4(1.0, 0.0, 0.0, x,
                 0.0, 1.0, 0.0, y,
                 0.0, 0.0, 1.0, z,
                 0.0, 0.0, 0.0, 1.0 )
}

pub fn scaling(x : Float , y: Float, z : Float) -> Matrix<4> {
    Matrix::new4(x, 0.0, 0.0, 0.0,
                 0.0, y, 0.0, 0.0,
                 0.0, 0.0, z, 0.0,
                 0.0, 0.0, 0.0, 1.0 )
}

pub fn rotation_x(r : Float) -> Matrix<4> {
    let sin = r.sin();
    let cos = r.cos();

    Matrix::new4(1.0, 0.0, 0.0, 0.0,
                 0.0, cos, -sin, 0.0,
                 0.0, sin, cos, 0.0,
                 0.0, 0.0, 0.0, 1.0 )
}

pub fn rotation_y(r : Float) -> Matrix<4> {
    let sin = r.sin();
    let cos = r.cos();

    Matrix::new4(cos, 0.0, sin, 0.0,
                 0.0, 1.0, 0.0, 0.0,
                 -sin, 0.0, cos, 0.0,
                 0.0, 0.0, 0.0, 1.0 )
}

pub fn rotation_z(r : Float) -> Matrix<4> {
    let sin = r.sin();
    let cos = r.cos();

    Matrix::new4(cos, -sin, 0.0, 0.0,
                 sin, cos, 0.0, 0.0,
                 0.0, 0.0, 1.0, 0.0,
                 0.0, 0.0, 0.0, 1.0 )
}

pub fn shearing(xy : Float , xz: Float, yx : Float, yz : Float , zx: Float, zy : Float) -> Matrix<4> {
    Matrix::new4(1.0, xy, xz, 0.0,
                 yx, 1.0, yz, 0.0,
                 zx, zy, 1.0, 0.0,
                 0.0, 0.0, 0.0, 1.0 )
}

pub fn view_transform(from : Tuple, to : Tuple, up: Tuple) -> Matrix<4> {
    let forward = (to - from).normalize();
    let left = forward * &up.normalize();
    let true_up = left * &forward;

    let orientation = Matrix::new4(left.x, left.y, left.z, 0.0,
                 true_up.x, true_up.y, true_up.z, 0.0,
                 -forward.x, -forward.y, -forward.z, 0.0,
                 0.0, 0.0, 0.0, 1.0 );
    let translation = translation(-from.x, -from.y, -from.z);

    &orientation * &translation
}