pub(crate) type Float = f64;
pub(crate) const EPSILON: Float = 0.0001;
pub(crate) const INFINITY: Float = f64::INFINITY;
pub(crate) const PI : Float = std::f64::consts::PI;
pub(crate) const SQRT2 : Float = std::f64::consts::SQRT_2 as Float;

pub fn equals(a : Float, b : Float) -> bool {
    let delta_abs : Float = (a - b).abs();
    delta_abs < EPSILON
}

pub(crate) fn max(a: Float, b: Float, c: Float) -> Float {
    f64::max(f64::max(a, b), c)
}

pub(crate) fn min(a: Float, b: Float, c: Float) -> Float {
    f64::min(f64::min(a, b), c)
}