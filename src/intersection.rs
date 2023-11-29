use crate::math::Float;
use crate::shape::{Shape};

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t : Float,
    pub shape: &'a Shape
}

impl Intersection<'_> {
    pub fn new(t : Float, shape : &Shape) -> Intersection {
        let inter = Intersection {t, shape};
        inter
    }
}