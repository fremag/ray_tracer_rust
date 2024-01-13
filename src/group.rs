use crate::intersections::{Intersections, intersections};
use crate::matrix::Matrix;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::{Tuple, vector};

#[derive(Debug)]
pub struct Group<'a> {
    children : Vec<&'a Object<'a>>,
}

impl<'a> Group<'a> {
    pub(crate) fn intersect(&self, ray: &Ray) -> Intersections {

        let mut xs = intersections(vec![]);
        for child in self.children.iter() {
            let child_xs = child.intersect(ray);
            for x in child_xs.intersections.iter() {
                xs.intersections.push(*x);
            }
        }

        xs
    }

    pub(crate) fn normal_at(&self, object_point: Tuple) -> Tuple {
        // TODO
        vector(0.0, 0.0, 0.0)
    }

    pub(crate) fn set_transformation(&self, transformation: Matrix<4>) {
        // TODO
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