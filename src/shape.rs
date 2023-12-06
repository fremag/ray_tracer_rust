use std::fmt::Debug;
use crate::intersections::Intersections;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub enum Shape {Sphere(Sphere)}

impl Shape {
    pub(crate) fn normal_at(&self, p: Tuple) -> Tuple {
        match self {
            Shape::Sphere(sphere) => sphere.normal_at(p)
        }
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Shape::Sphere(_), Shape::Sphere(_)) =>true
        }
    }
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray)
        }
    }
}
