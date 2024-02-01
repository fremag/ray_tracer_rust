use crate::colors::Color;
use crate::core::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct StripePattern {
    pub a : Color,
    pub b : Color,
}

impl StripePattern {
    pub fn new(a: Color, b : Color) -> Self {
        Self {a, b}
    }

    pub fn pattern_at(&self, point: &Tuple) -> Color {
        let x = point.x.floor() as i32 % 2;
        if x == 0 {
            return self.a
        }

        self.b
    }
}
