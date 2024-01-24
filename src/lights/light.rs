use crate::colors::Color;
use crate::lights::point_light::PointLight;
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