use crate::colors::Color;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::patterns::checker::CheckerPattern;
use crate::patterns::gradient::GradientPattern;
use crate::patterns::ring::RingPattern;
use crate::patterns::stripe::StripePattern;
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

    pub(crate) fn gradient(color_a: Color, color_b: Color) -> Pattern {
        Self::from(Patterns::Gradient(GradientPattern::new(color_a, color_b)))
    }

    pub(crate) fn ring(color_a: Color, color_b: Color) -> Pattern {
        Self::from(Patterns::Ring(RingPattern::new(color_a, color_b)))
    }

    pub(crate) fn checker(color_a: Color, color_b: Color) -> Pattern {
        Self::from(Patterns::Checker(CheckerPattern::new(color_a, color_b)))
    }

    pub(crate) fn test() -> Pattern {
        Self::from(Patterns::Test)
    }

    pub(crate) fn pattern_at_object(&self, object: &Object, world_point: Tuple) -> Color {
        let object_point = object.transformation_inverse() * &world_point;
        let pattern_point = &self.inverse_transform * &object_point;
        match self.pattern {
            Patterns::None => {object.material().color}
            Patterns::Stripe(stripes) => {stripes.pattern_at(&pattern_point)}
            Patterns::Test => {Color::new(pattern_point.x, pattern_point.y, pattern_point.z)}
            Patterns::Gradient(gradient) => {gradient.pattern_at(&pattern_point)}
            Patterns::Ring(ring) => {ring.pattern_at(&pattern_point)}
            Patterns::Checker(checker) => {checker.pattern_at(&pattern_point)}
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
    Test,
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checker(CheckerPattern)
}
