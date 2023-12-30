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

#[derive(Debug, Copy, Clone)]
pub struct GradientPattern {
    pub a : Color,
    pub b : Color,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self { Self {a, b} }

    pub(crate) fn pattern_at(&self, point: &Tuple) -> Color {
        let fraction = point.x - point.x.floor();
        Color::new(
            self.a.r + (self.b.r - self.a.r) * fraction,
            self.a.g + (self.b.g - self.a.g) * fraction,
            self.a.b + (self.b.b - self.a.b) * fraction
        )
    }
}

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

#[derive(Debug, Copy, Clone)]
pub struct CheckerPattern {
    pub a : Color,
    pub b : Color,
}

impl CheckerPattern {
    pub fn new(a: Color, b: Color) -> Self { Self {a, b} }

    pub(crate) fn pattern_at(&self, point: &Tuple) -> Color {
        let sum_floor_coord = point.x.floor() as i32  + point.y.floor() as i32 + point.z.floor()  as i32;
        if sum_floor_coord % 2 == 0 {
            return self.a
        }

        self.b
    }
}
