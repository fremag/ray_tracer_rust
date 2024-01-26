use crate::core::bounds::Bounds;
use crate::core::intersections::{Intersections, intersections};
use crate::core::matrix::Matrix;
use crate::object::Object;
use crate::core::ray::Ray;

#[derive(Debug, Clone)]
pub struct Group {
    children : Vec<Object>,
    bounds : Bounds,
    transformation : Matrix<4>
}

impl Group {
    pub(crate) fn bounds(&self) -> Bounds {
        self.bounds.clone()
    }

    pub(crate) fn child(&self, p0: usize) -> &Object {
        &self.children[p0]
    }

    pub fn new() -> Self {
        Self {  children: vec![],
                bounds: Bounds::new(),
                transformation: Matrix::<4>::identity()}
    }

    pub(crate) fn set_transformation(&mut self, transformation: Matrix<4>) {
        let mut transformed_objects: Vec<Object> = vec![];
        let mut bounds = Bounds::new();
        for object in self.children.iter_mut() {
            object.set_transformation(&transformation * object.transformation());
            let transformed_object = object.clone();
            let transformed_bounds = transformed_object.bounds();
            bounds.extend(&transformed_bounds);
            transformed_objects.push(transformed_object);
        }

        self.transformation = transformation;
        self.children = transformed_objects;
        self.bounds = bounds;
    }

    pub fn from(objects: Vec<Object>, transformation: Matrix<4>) -> Object {
        let mut group = Group::new();
        group.set_transformation(transformation);
        objects.iter().for_each( |a|  { group.add(a.clone())});
        Object::new_group(group)
    }

    pub fn add(&mut self, child: Object) {
        let mut transformed_children = child.clone();
        transformed_children.set_transformation(&self.transformation * child.transformation());
        if self.children.is_empty() {
            self.bounds = transformed_children.bounds();
        } else {
            self.bounds.extend(&transformed_children.bounds());
        }
        self.children.push(transformed_children);

    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Intersections {
        let mut xs = intersections(vec![]);
        if self.bounds.intersect(&ray).is_empty() {
            return xs;
        }

        for child in self.children.iter() {
            let child_xs = child.intersect(ray);
            for x in child_xs.intersections.iter() {
                xs.intersections.push(*x);
            }
        }

        xs
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }
}