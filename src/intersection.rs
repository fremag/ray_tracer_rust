use crate::math::Float;
use crate::shape::{Shape};

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t : Float,
    pub shape: &'a Shape
}

impl<'a> Intersection<'a> {
    pub fn new(t : Float, shape : &'a Shape) -> Self {
        let inter = Intersection {t, shape};
        inter
    }
}