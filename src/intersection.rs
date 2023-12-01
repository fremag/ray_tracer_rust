use crate::math::Float;
use crate::shape::{Shape};

#[derive(Debug)]
pub struct Intersection {
    pub t : Float,
    pub shape: Shape
}

impl Intersection {
    pub fn new(t : Float, shape : Shape) -> Self {
        let inter = Intersection {t, shape};
        inter
    }
}