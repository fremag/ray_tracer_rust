use crate::colors::Color;
use crate::math::{equals, Float};

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub color : Color,
    pub ambient : Float,
    pub diffuse : Float,
    pub specular : Float,
    pub shininess : Float,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color.eq(&other.color)
        && equals(self.ambient, other.ambient)
        && equals(self.diffuse, other.diffuse)
        && equals(self.specular, other.specular)
        && equals(self.shininess, other.shininess)
    }
}

pub fn material() -> Material {
    Material { color : Color {r: 1.0, g: 1.0, b: 1.0}, ambient : 0.1, diffuse: 0.9, specular: 0.9, shininess: 200.0}
}

