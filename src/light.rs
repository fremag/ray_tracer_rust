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

impl PartialEq<Self> for Light {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Light::PointLight(point_light),Light::PointLight(other_point_light) ) => point_light.eq(other_point_light)
        }
    }
}

impl Eq for Light {

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

impl PartialEq<Self> for PointLight {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.intensity == other.intensity
    }
}

impl Eq for PointLight {

}