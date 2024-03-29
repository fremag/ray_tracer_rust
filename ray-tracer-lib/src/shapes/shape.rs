use std::fmt::Debug;
use crate::core::bounds::Bounds;
use crate::core::intersection::Intersection;
use crate::shapes::cone::Cone;
use crate::shapes::cube::Cube;
use crate::shapes::cylinder::Cylinder;
use crate::core::math::Float;
use crate::shapes::plane::Plane;
use crate::core::ray::Ray;
use crate::shapes::sphere::Sphere;
use crate::core::tuple::{Tuple};
use crate::shapes::smooth_triangle::SmoothTriangle;
use crate::shapes::triangle::Triangle;

#[derive(Debug, Clone)]
pub enum Shape {Sphere(Sphere), Plane(Plane), Cube(Cube), Cylinder(Cylinder), Cone(Cone), Triangle(Triangle), SmoothTriangle(SmoothTriangle)}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Shape::Sphere(_), Shape::Sphere(_)) => true,
            (Shape::Plane(_), Shape::Plane(_)) => true,
            (Shape::Cube(_), Shape::Cube(_)) => true,
            (Shape::Cylinder(cyl1), Shape::Cylinder(cyl2)) => cyl1.eq(cyl2),
            (Shape::Cone(cone1), Shape::Cone(cone2)) => cone1.eq(cone2),
            (Shape::Triangle(triangle1), Shape::Triangle(triangle2)) => triangle1.eq(triangle2),
            (Shape::SmoothTriangle(smooth_triangle1), Shape::SmoothTriangle(smooth_triangle2)) => smooth_triangle1.eq(smooth_triangle2),
            _ => false
        }
    }
}

impl Shape {
    pub fn bounds(&self) -> Bounds {
        match self {
            Shape::Sphere(sphere) => sphere.bounds(),
            Shape::Plane(plane) => plane.bounds(),
            Shape::Cube(cube) => cube.bounds(),
            Shape::Cylinder(cylinder) => cylinder.bounds(),
            Shape::Cone(cone) => cone.bounds(),
            Shape::Triangle(triangle) => triangle.bounds(),
            Shape::SmoothTriangle(smooth_triangle) => smooth_triangle.bounds()
        }
    }

    pub fn normal_at(&self, p: Tuple, hit: &Intersection) -> Tuple {
        match self {
            Shape::Sphere(sphere) => sphere.normal_at(p),
            Shape::Plane(plane) => plane.normal_at(p),
            Shape::Cube(cube) => cube.normal_at(p),
            Shape::Cylinder(cylinder) => cylinder.normal_at(&p),
            Shape::Cone(cone) => cone.normal_at(&p),
            Shape::Triangle(triangle) => triangle.normal_at(&p),
            Shape::SmoothTriangle(smooth_triangle) => smooth_triangle.normal_at(&p, hit),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Float> {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray),
            Shape::Plane(plane) => plane.intersect(ray),
            Shape::Cube(cube) => cube.intersect(ray),
            Shape::Cylinder(cyl) => cyl.intersect(ray),
            Shape::Cone(cone) => cone.intersect(ray),
            Shape::Triangle(triangle) => triangle.intersect(ray),
            Shape::SmoothTriangle(smooth_triangle) => smooth_triangle.intersect(ray),
        }
    }
}
