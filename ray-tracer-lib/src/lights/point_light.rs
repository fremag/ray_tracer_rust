use crate::colors::Color;
use crate::lights::light::Light;
use crate::core::tuple::Tuple;

pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color
}

impl PointLight {
    pub fn new(position : Tuple, intensity : Color) -> Light {
        Light::PointLight(PointLight{position, intensity})
    }
}

impl PartialEq<Self> for PointLight {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.intensity == other.intensity
    }
}

impl Eq for PointLight {

}
