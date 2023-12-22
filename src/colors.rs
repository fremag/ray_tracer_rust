use crate::math::{equals, Float};
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Color  {
    pub(crate) r : Float,
    pub(crate) g : Float,
    pub(crate) b : Float
}

impl Color {
    pub fn black() -> Color { Color::new(0.0, 0.0, 0.0) }
    pub fn white() -> Color { Color::new(1.0, 1.0, 1.0) }
    pub fn red() -> Color { Color::new(1.0, 0.0, 0.0) }
    pub fn green() -> Color { Color::new(0.0, 1.0, 0.0) }
    pub fn blue() -> Color { Color::new(0.0, 0.0, 1.0) }

    pub fn new(red : Float, green : Float, blue : Float ) -> Self {
        Self{r: red, g: green, b: blue}
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        equals(self.r, other.r) && equals(self.g, other.g) && equals(self.b, other.b)
    }
}
impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color { r: self.r+ rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
    }
}
impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color { r: self.r - rhs.r, g: self.g - rhs.g, b: self.b - rhs.b }
    }
}

impl Mul<Float> for Color {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        Color { r: self.r * rhs, g: self.g * rhs, b: self.b * rhs }
    }
}


impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color { r: self.r * rhs.r, g: self.g * rhs.g, b: self.b * rhs.b }
    }
}

