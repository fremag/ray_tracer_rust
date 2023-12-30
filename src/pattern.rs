use crate::colors::Color;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Pattern {
    pub(crate) pattern : Patterns,
    pub inverse_transform : Matrix<4>,
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern { pattern: Patterns::None, inverse_transform: Matrix::<4>::identity()}
    }

    pub fn from(pattern : Patterns) -> Self {
        let mut new_pattern = Self::new();
        new_pattern.pattern = pattern;
        new_pattern
    }

    pub(crate) fn stripe(color_a: Color, color_b: Color) -> Pattern {
        Self::from(Patterns::Stripe(StripePattern::new(color_a, color_b)))
    }

    pub(crate) fn test() -> Pattern {
        Self::from(Patterns::Test)
    }

    pub(crate) fn pattern_at_object(&self, object: &Object, world_point: Tuple) -> Color {
        let object_point = object.transformation_inverse() * &world_point;
        let pattern_point = &self.inverse_transform * &object_point;
        match self.pattern {
            Patterns::None => {object.material().color}
            Patterns::Stripe(stripes) => {stripes.stripe_at(&pattern_point)}
            Patterns::Test => {Color::new(pattern_point.x, pattern_point.y, pattern_point.z)}
        }
    }

    pub(crate) fn set_pattern_transform(&mut self, transform: &Matrix<4>) {
        self.inverse_transform = transform.inverse();
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Patterns {
    None,
    Stripe(StripePattern),
    Test
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

