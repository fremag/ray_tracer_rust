use crate::colors::Color;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Pattern {
    pub(crate) pattern : Patterns,
    transform : Matrix<4>,
    inverse_transform : Matrix<4>,
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern { pattern: Patterns::None, transform : Matrix::<4>::identity(), inverse_transform: Matrix::<4>::identity()}
    }

    pub fn from(pattern : Patterns) -> Self {
        let mut new_pattern = Self::new();
        new_pattern.pattern = pattern;
        new_pattern
    }

    pub(crate) fn stripe(color_a: Color, color_b: Color) -> Pattern {
        Self::from(Patterns::Stripe(StripePattern::new(color_a, color_b)))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Patterns {
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