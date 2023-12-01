use std::fmt::Debug;
use crate::intersections::Intersections;
use crate::ray::Ray;
use crate::sphere::Sphere;

#[derive(Debug, Copy, Clone)]
pub enum Shape {Sphere(Sphere)}

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
