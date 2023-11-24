use crate::math::Float;
use crate::sphere::Sphere;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t : Float,
    pub shape: &'a Sphere
}

impl Intersection<'_> {
    pub fn new(t : Float, shape : &Sphere) -> Intersection {
        let inter = Intersection {t, shape};
        inter
    }
}