use crate::colors::Color;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub enum Pattern {
    None,
    Stripe(StripePattern)
}

#[derive(Debug, Copy, Clone)]
pub struct StripePattern {
    pub a : Color,
    pub b : Color,
}

impl StripePattern {
    pub fn new(a: Color, b : Color) -> Self {
        StripePattern {a, b}
    }

    pub fn stripe_at(&self, point: &Tuple) -> Color {
        let x = point.x.floor() as i32 % 2;
        if x == 0 {
            return self.a
        }

        self.b
    }
}