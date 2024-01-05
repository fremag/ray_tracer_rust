use crate::math::Float;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::{Tuple, vector};

#[derive(Debug)]
pub struct Group<'a> {
    pub children : Vec<Object<'a>>,
}

impl<'a> Group<'a> {
    pub fn add(&mut self, object: Object<'a>) {
        self.children.push(object);
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<Float> {
        vec![]
    }

    pub(crate) fn normal_at(&self, point: &Tuple) -> Tuple {
        vector(0.0, 0.0, 0.0)
    }
}