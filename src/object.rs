use crate::cone::Cone;
use crate::cube::{Cube};
use crate::cylinder::Cylinder;
use crate::intersection::Intersection;
use crate::intersections::{Intersections, intersections};
use crate::material::{Material};
use crate::math::Float;
use crate::matrix::Matrix;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::sphere;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Object<'a> {
    shape: Shape,
    parent: Option<&'a Object<'a>>,
    material: Material,
    transformation: Matrix<4>,
    transformation_inverse: Matrix<4>,
    // optimization: keep inverse transformation
    transformation_inverse_transpose: Matrix<4>, // optimization: keep inverse transformation transpose
    children : Vec<&'a Object<'a>>,
}

impl<'a> Object<'a> {
    pub(crate) fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = &(self.transformation_inverse) * &world_point;
        let object_normal = self.shape.normal_at(object_point);
        let mut world_normal = &(self.transformation_inverse_transpose) * &object_normal;
        world_normal.w = 0.0;
        let n = world_normal.normalize();
        n
    }

    pub fn add(mut self, child: &'a Object<'a>) {
        let shape = &mut self.shape;

        match shape {
            Shape::Group(_) => { self.children.push(child)},
            _ => {}
        }
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Intersections {
        let ray2 = ray.transform(&self.transformation_inverse);
        let vec = self.shape.intersect(&ray2).iter().map(|t| Intersection { t: *t, object: &self }).collect();
        intersections(vec)
    }

    pub fn set_transformation(&mut self, transformation: Matrix<4>) -> &Self {
        self.transformation = transformation;
        self.transformation_inverse = self.transformation.inverse();
        self.transformation_inverse_transpose = self.transformation_inverse.transpose();
        self
    }

    pub fn transformation(&self) -> &Matrix<4> {
        &self.transformation
    }

    pub fn transformation_inverse(&self) -> &Matrix<4> {
        &self.transformation_inverse
    }
    pub fn shape(&self) -> &Shape { &self.shape }
    pub fn material(&self) -> &Material { &(self.material) }
    pub fn set_material(&mut self, material: Material) -> &Self {
        self.material = material;
        self
    }

    pub fn new(shape: Shape) -> Object<'a> {
        Object {
            shape,
            material: Material::new(),
            parent: None,
            children: vec![],
            transformation: Matrix::<4>::identity(),
            transformation_inverse: Matrix::<4>::identity(),
            transformation_inverse_transpose: Matrix::<4>::identity(),
        }
    }
}

impl PartialEq for Object<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape && self.transformation == other.transformation && self.material == other.material
    }
}

pub fn build_sphere() -> Object<'static> {
    let sphere = sphere();
    Object::new(sphere)
}

pub fn build_plane() -> Object<'static> {
    let plane = Shape::Plane(Plane::new());
    Object::new(plane)
}

pub fn build_cube() -> Object<'static> {
    let cube = Shape::Cube(Cube::new());
    Object::new(cube)
}

pub fn build_cylinder(min: Float, max: Float) -> Object<'static> {
    let cyl = Shape::Cylinder(Cylinder::from(min, max, true));
    Object::new(cyl)
}

pub fn build_cone(min: Float, max: Float) -> Object<'static> {
    let cone = Shape::Cone(Cone::from(min, max, true));
    Object::new(cone)
}

pub fn build_glass_sphere() -> Object<'static> {
    let mut sphere = build_sphere();
    let mut material = Material::new();
    material.transparency = 1.0;
    material.refractive_index = 1.5;
    sphere.set_material(material);
    sphere
}
