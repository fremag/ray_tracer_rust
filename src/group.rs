use crate::math::Float;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::{Tuple, vector};

#[derive(Debug)]
pub struct Group<'a> {
    children : Vec<&'a Object<'a>>,
}

impl<'a> Group<'a> {
    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<Float> {
        vec![]
    }

    pub(crate) fn normal_at(&self, object_point: Tuple) -> Tuple {
        vector(0.0, 0.0, 0.0)
    }

    pub fn add(&mut self, child: &'a Object<'a>) {
        self.children.push(child);
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn new() -> Self {
        Self { children: vec![]}
    }
}