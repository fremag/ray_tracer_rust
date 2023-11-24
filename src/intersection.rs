use crate::math::Float;
use crate::shape::Shape;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t : Float,
    pub shape: &'a dyn Shape
}

impl Intersection<'_> {
    pub fn new(t : Float, shape : & dyn Shape) -> Intersection {
        let inter = Intersection {t, shape};
        inter
    }
}