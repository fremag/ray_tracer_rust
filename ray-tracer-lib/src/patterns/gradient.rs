use crate::colors::Color;
use crate::core::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct GradientPattern {
    pub a : Color,
    pub b : Color,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self { Self {a, b} }

    pub fn pattern_at(&self, point: &Tuple) -> Color {
        let fraction = point.x - point.x.floor();
        Color::new(
            self.a.r + (self.b.r - self.a.r) * fraction,
            self.a.g + (self.b.g - self.a.g) * fraction,
            self.a.b + (self.b.b - self.a.b) * fraction
        )
    }
}
