use crate::core::matrix::Matrix;
use crate::core::tuple::Tuple;

pub type Float = f64;

pub const EPSILON: Float = 0.0001;
pub const NAN: Float = f64::NAN;
pub const INFINITY: Float = f64::INFINITY;
pub const PI: Float = std::f64::consts::PI;
pub const SQRT2: Float = std::f64::consts::SQRT_2 as Float;

pub fn equals(a: Float, b: Float) -> bool {
    let delta_abs: Float = (a - b).abs();
    delta_abs < EPSILON
}

pub fn max(a: Float, b: Float, c: Float) -> Float {
    f64::max(f64::max(a, b), c)
}

pub fn min(a: Float, b: Float, c: Float) -> Float {
    f64::min(f64::min(a, b), c)
}


pub fn rotation(u: Tuple, v: Tuple) -> Matrix<4> {
    let cos_phi = u.dot(&v);
    let uv = u * &v;
    let uv_magnitude = u.magnitude() * v.magnitude();
    let sin_phi = uv.magnitude() / uv_magnitude;
    // https://en.wikipedia.org/wiki/Rodrigues%27_rotation_formula
    let m1 = Matrix::<4>::new(
        [[cos_phi, 0.0, 0.0, 0.0],
            [0.0, cos_phi, 0.0, 0.0],
            [0.0, 0.0, cos_phi, 0.0],
            [0.0, 0.0, 0.0, 1.0]]
    );

    if sin_phi.abs() < Float::EPSILON
    {
        return m1;
    }

    let n = uv / sin_phi;
    let m2 = Matrix::<4>::new(
        [[n.x * n.x, n.x * n.y, n.x * n.z, 0.0],
            [n.x * n.y, n.y * n.y, n.y * n.z, 0.0],
            [n.x * n.z, n.z * n.y, n.z * n.z, 0.0],
            [0.0, 0.0, 0.0, 0.0]]
    );

    let m3 = Matrix::<4>::new(
        [[0.0, -n.z, n.y, 0.0],
            [n.z, 0.0, -n.x, 0.0],
            [-n.y, n.x, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]]);
    let m2p = &m2 * (1.0 - cos_phi);
    let m3p= &m3 * sin_phi;
    let m = &m1 + &(&m2p + &m3p);
    m
}

