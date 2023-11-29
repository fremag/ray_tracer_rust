use std::fmt::Debug;
use crate::math::Float;
use crate::ray::Ray;
use crate::sphere::Sphere;

#[derive(Debug)]
pub enum Shape {Sphere(Sphere)}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Shape::Sphere(_), Shape::Sphere(_)) =>true
        }
    }
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Vec<Float> {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray)
        }
    }
}
