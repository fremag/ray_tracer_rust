use ObjectType::ObjectGroup;
use crate::core::bounds::Bounds;
use crate::shapes::cone::Cone;
use crate::shapes::cube::{Cube};
use crate::shapes::cylinder::Cylinder;
use crate::shapes::group::Group;
use crate::core::intersection::Intersection;
use crate::core::intersections::{Intersections, intersections};
use crate::material::{Material};
use crate::core::math::Float;
use crate::core::matrix::Matrix;
use crate::object::ObjectType::ObjectShape;
use crate::shapes::plane::Plane;
use crate::core::ray::Ray;
use crate::shapes::shape::Shape;
use crate::shapes::sphere::sphere;
use crate::core::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Object {
    object_type: ObjectType,
    material: Material,
    transformation: Matrix<4>,
    transformation_inverse: Matrix<4>,    // optimization: keep inverse transformation
    transformation_inverse_transpose: Matrix<4>, // optimization: keep inverse transformation transpose
}

#[derive(Debug, Clone)]
pub enum ObjectType { ObjectShape(Shape), ObjectGroup(Group)}

impl Object {
    pub(crate) fn normal_to_world(&self, normal: &Tuple) -> Tuple {
        let mut n = &self.transformation_inverse_transpose * normal;
        n.w = 0.0;
        n.normalize()
    }

    pub(crate) fn world_to_object(&self, point: &Tuple) -> Tuple {
        &(self.transformation_inverse) * point
    }

    pub(crate) fn normal_at(&self, world_point: Tuple) -> Tuple {
        let local_point = self.world_to_object(&world_point);
        let local_normal = match &self.object_type {
            ObjectShape(shape) => shape.normal_at(local_point),
            ObjectGroup(_) => panic!("No !"),
        };
        let n = self.normal_to_world(&local_normal);
        n
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Intersections {
        let ray2 = ray.transform(&self.transformation_inverse);
        return match &self.object_type {
            ObjectShape(shape) => intersections(shape.intersect(&ray2).iter().map(|t| Intersection { t: *t, object: &self }).collect()),
            ObjectGroup(group) => group.intersect(&ray2),
        };
    }

    pub(crate) fn bounds(&self) -> Bounds {
        return match &self.object_type {
            ObjectShape(shape) => shape.bounds().transform(&self.transformation),
            ObjectGroup(group) => group.bounds(),
        };
    }

    pub fn set_transformation(&mut self, transformation: Matrix<4>) -> &Self {

        match &mut self.object_type {
            ObjectGroup(group) => {
                group.set_transformation(transformation);
            },
            ObjectShape(_) => {
                self.transformation = transformation;
                self.transformation_inverse = self.transformation.inverse();
                self.transformation_inverse_transpose = self.transformation_inverse.transpose();
            }
        }
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

    pub fn new(shape: Shape) -> Object {
        Object {
            object_type: ObjectShape(shape),
            material: Material::new(),
            transformation: Matrix::<4>::identity(),
            transformation_inverse: Matrix::<4>::identity(),
            transformation_inverse_transpose: Matrix::<4>::identity(),
        }
    }

    pub fn new_group(group: Group) -> Object {
        Object {
            object_type: ObjectGroup(group),
            material: Material::new(),
            transformation: Matrix::<4>::identity(),
            transformation_inverse: Matrix::<4>::identity(),
            transformation_inverse_transpose: Matrix::<4>::identity(),
        }
    }
}

impl PartialEq for Object {
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

pub fn build_sphere() -> Object {
    let sphere = sphere();
    Object::new(sphere)
}

pub fn build_plane() -> Object {
    let plane = Shape::Plane(Plane::new());
    Object::new(plane)
}

pub fn build_cube() -> Object {
    let cube = Shape::Cube(Cube::new());
    Object::new(cube)
}

pub fn build_cylinder(min: Float, max: Float) -> Object {
    let cyl = Shape::Cylinder(Cylinder::from(min, max, true));
    Object::new(cyl)
}

pub fn build_cone(min: Float, max: Float) -> Object {
    let cone = Shape::Cone(Cone::from(min, max, true));
    Object::new(cone)
}

pub fn build_glass_sphere() -> Object {
    let mut sphere = build_sphere();
    let mut material = Material::new();
    material.transparency = 1.0;
    material.refractive_index = 1.5;
    sphere.set_material(material);
    sphere
}
