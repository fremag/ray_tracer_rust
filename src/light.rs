use crate::colors::Color;
use crate::tuple::Tuple;

pub enum Light {PointLight(PointLight)}

impl Light {
    pub fn intensity(&self) -> Color {
        match self {
            Light::PointLight(point_light) => point_light.intensity
        }
    }

    pub fn position(&self) -> Tuple {
        match self {
            Light::PointLight(point_light) => point_light.position
        }
    }
}

pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color
}

impl PointLight {
    pub fn new(position : Tuple, intensity : Color) -> Light {
        Light::PointLight(PointLight{position, intensity})
    }
}