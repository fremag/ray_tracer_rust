use crate::intersections::{Intersections, intersections};
use crate::matrix::Matrix;
use crate::object::Object;
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct Group {
    children : Vec<Object>,
}

impl Group {
    pub(crate) fn child(&self, p0: usize) -> &Object {
        &self.children[p0]
    }
}

impl Group {
    pub(crate) fn set_transformation(&mut self, transformation: Matrix<4>) {
        let mut transformed_children : Vec<Object> = vec![];
        for object in self.children.iter_mut() {
            object.set_transformation(&transformation * object.transformation());
            transformed_children.push(object.clone());
        }

        self.children = transformed_children;
    }
}

impl Group {

    pub fn from(mut objects: Vec<Object>, transformation: Matrix<4>) -> Object {
        let mut group = Group::new();
        for object in objects.iter_mut() {
            object.set_transformation(&transformation * object.transformation());
            group.add(object.clone());
        }
        Object::new_group(group)
    }

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

    pub fn add(&mut self, child: Object) {
        self.children.push(child);
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn new() -> Self {
        Self { children: vec![]}
    }
}