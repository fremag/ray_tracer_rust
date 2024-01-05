use std::fmt::Debug;
use crate::cone::Cone;
use crate::cube::Cube;
use crate::cylinder::Cylinder;
use crate::group::Group;
use crate::math::Float;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

#[derive(Debug)]
pub enum Shape<'a> {Sphere(Sphere), Plane(Plane), Cube(Cube), Cylinder(Cylinder), Cone(Cone), Group(Group<'a>)}

impl Shape<'_> {
    pub(crate) fn normal_at(&self, p: Tuple) -> Tuple {
        match self {
            Shape::Sphere(sphere) => sphere.normal_at(p),
            Shape::Plane(plane) => plane.normal_at(p),
            Shape::Cube(cube) => cube.normal_at(p),
            Shape::Cylinder(cylinder) => cylinder.normal_at(&p),
            Shape::Cone(cone) => cone.normal_at(&p),
            Shape::Group(group) => group.normal_at(&p),
        }
    }
}

impl PartialEq for Shape<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Shape::Sphere(_), Shape::Sphere(_)) => true,
            (Shape::Plane(_), Shape::Plane(_)) => true,
            (Shape::Cube(_), Shape::Cube(_)) => true,
            (Shape::Cylinder(cyl1), Shape::Cylinder(cyl2)) => cyl1.eq(cyl2),
            (Shape::Cone(cone1), Shape::Cone(cone2)) => cone1.eq(cone2),
            _ => false
        }
    }
}

impl Shape<'_> {
    pub fn intersect(&self, ray: &Ray) -> Vec<Float> {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray),
            Shape::Plane(plane) => plane.intersect(ray),
            Shape::Cube(cube) => cube.intersect(ray),
            Shape::Cylinder(cyl) => cyl.intersect(ray),
            Shape::Cone(cone) => cone.intersect(ray),
            Shape::Group(group) => group.intersect(ray),
        }
    }
}
