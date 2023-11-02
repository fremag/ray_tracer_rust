pub(crate) type Float = f32;
const EPSILON: Float = 0.0001;

pub fn equals(a : Float, b : Float) -> bool {
    let delta= a - b;
    delta.abs() < EPSILON
}