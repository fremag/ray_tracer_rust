use crate::colors::Color;
use crate::core::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct RingPattern {
    pub a : Color,
    pub b : Color,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self { Self {a, b} }

    pub(crate) fn pattern_at(&self, point: &Tuple) -> Color {
        let dist = (point.x * point.x + point.z * point.z).sqrt().floor() as i32;
        if dist % 2 == 0 {
            return self.a
        }

        self.b
    }
}
