pub(crate) type Float = f32;
pub(crate) const EPSILON: Float = 0.0001;
pub(crate) const PI : Float = std::f32::consts::PI;
pub(crate) const SQRT2 : Float = std::f32::consts::SQRT_2 as Float;

pub fn equals(a : Float, b : Float) -> bool {
    let delta_abs : Float = (a - b).abs();
    delta_abs < EPSILON
}