use ObjectType::ObjectGroup;
use crate::cone::Cone;
use crate::cube::{Cube};
use crate::cylinder::Cylinder;
use crate::group::Group;
use crate::intersection::Intersection;
use crate::intersections::{Intersections, intersections};
use crate::material::{Material};
use crate::math::Float;
use crate::matrix::Matrix;
use crate::object::ObjectType::ObjectShape;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::sphere;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Object<'a> {
    object_type: ObjectType<'a>,
    material: Material,
    transformation: Matrix<4>,
    transformation_inverse: Matrix<4>,    // optimization: keep inverse transformation
    transformation_inverse_transpose: Matrix<4>, // optimization: keep inverse transformation transpose
}

#[derive(Debug)]
pub enum ObjectType<'a> { ObjectShape(Shape), ObjectGroup(Group<'a>)}

impl<'a> Object<'a> {
    pub(crate) fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = &(self.transformation_inverse) * &world_point;
        let object_normal = match &self.object_type {
            ObjectShape(shape) => shape.normal_at(object_point),
            ObjectGroup(group) => group.normal_at(object_point),
        };
        let mut world_normal = &(self.transformation_inverse_transpose) * &object_normal;
        world_normal.w = 0.0;
        let n = world_normal.normalize();
        n
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Intersections {
        let ray2 = ray.transform(&self.transformation_inverse);
        let vec = match &self.object_type {
            ObjectShape(shape) => shape.intersect(&ray2),
            ObjectGroup(group) => group.intersect(&ray2),
        };
        let all_intersections = vec.iter().map(|t| Intersection { t: *t, object: &self }).collect();
        intersections(all_intersections)
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
    pub fn shape(&self) -> Option<&Shape> { match &self.object_type {
        ObjectShape(shape) => Some(&shape),
        _ => None,
    }}
    pub fn group(&self) -> Option<&Group> { match &self.object_type {
        ObjectGroup(group) => Some(&group),
        _ => None,
    }}
    pub fn material(&self) -> &Material { &(self.material) }
    pub fn set_material(&mut self, material: Material) -> &Self {
        self.material = material;
        self
    }

    pub fn new(shape: Shape) -> Object<'a> {
        Object {
            object_type: ObjectShape(shape),
            material: Material::new(),
            transformation: Matrix::<4>::identity(),
            transformation_inverse: Matrix::<4>::identity(),
            transformation_inverse_transpose: Matrix::<4>::identity(),
        }
    }

    pub fn new_group(group: Group<'a>) -> Object<'a> {
        Object {
            object_type: ObjectGroup(group),
            material: Material::new(),
            transformation: Matrix::<4>::identity(),
            transformation_inverse: Matrix::<4>::identity(),
            transformation_inverse_transpose: Matrix::<4>::identity(),
        }
    }
}

impl PartialEq for Object<'_> {
    fn eq(&self, other: &Self) -> bool {
        if ! (self.transformation == other.transformation && self.material == other.material) {
            return false;
        }
        match (&self.object_type, &other.object_type) {
            (ObjectShape(shape1), ObjectShape(shape2)) => shape1 == shape2,
            (_, _) => false
        }
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
