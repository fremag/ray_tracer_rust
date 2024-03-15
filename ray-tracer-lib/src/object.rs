use std::sync::atomic::{AtomicUsize, Ordering};
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
use crate::object::ObjectType::{CsgGroup, ObjectShape, SmoothTriangleGroup, TriangleGroup};
use crate::shapes::plane::Plane;
use crate::core::ray::Ray;
use crate::shapes::shape::Shape;
use crate::shapes::sphere::sphere;
use crate::core::tuple::Tuple;
use crate::shapes::csg::Csg;
use crate::shapes::smooth_triangle_model::SmoothTriangleModel;
use crate::shapes::triangle_model::TriangleModel;

pub static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub static INTERSECTION_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub fn get_next_unique_shape_id() -> usize {
    OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[derive(Debug, Clone)]
pub struct Object {
    pub object_id : usize,
    pub object_type: ObjectType,
    material: Material,
    transformation: Matrix<4>,
    transformation_inverse: Matrix<4>,    // optimization: keep inverse transformation
    transformation_inverse_transpose: Matrix<4>, // optimization: keep inverse transformation transpose
}

#[derive(Debug, Clone)]
pub enum ObjectType { ObjectShape(Shape), ObjectGroup(Group), TriangleGroup(TriangleModel), SmoothTriangleGroup(SmoothTriangleModel), CsgGroup(Csg)}

impl Object {
    pub fn group(&self) -> Option<&Group> {
        match &self.object_type {
            ObjectGroup(group) => Some(&group),
            _ => None,
        }
    }

    pub fn normal_to_world(&self, normal: &Tuple) -> Tuple {
        let mut n = &self.transformation_inverse_transpose * normal;
        n.w = 0.0;
        n.normalize()
    }

    pub fn world_to_object(&self, point: &Tuple) -> Tuple {
        &(self.transformation_inverse) * point
    }

    pub fn normal_at(&self, world_point: Tuple, hit: &Intersection) -> Tuple {
        let local_point = self.world_to_object(&world_point);
        let local_normal = match &self.object_type {
            ObjectShape(shape) => shape.normal_at(local_point, hit),
            ObjectGroup(_) => panic!("No !"),
            TriangleGroup(_) => panic!("No !"),
            SmoothTriangleGroup(_) => panic!("No !"),
            CsgGroup(_) => panic!("No !"),
        };
        let n = self.normal_to_world(&local_normal);
        n
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        if cfg!(debug_assertions) {
            INTERSECTION_COUNTER.fetch_add(1, Ordering::SeqCst);
        }

        return match &self.object_type {
            ObjectShape(shape) => {
                let transformed_ray = ray.transform(&self.transformation_inverse);
                intersections(shape.intersect(&transformed_ray).iter().map(|t| Intersection::new(*t, self.clone() )).collect())
            },
            ObjectGroup(group) => group.intersect(&ray),
            TriangleGroup(model) => {

                let v = model.intersect(&ray).into_iter().map(|(t, triangle)| {
                    let id =  triangle.id;
                    let mut obj = Object::new_with_id(Shape::Triangle(triangle), id);
                    obj.set_material(self.material);
                    Intersection::new(t, obj)
                }).collect();
                intersections(v)
            },
            SmoothTriangleGroup(model) => {
                let v = model.intersect(&ray).into_iter().map(|(t, smooth_triangle, u, v)| {
                    let id  = smooth_triangle.triangle.id;
                    let mut obj = Object::new_with_id(Shape::SmoothTriangle(smooth_triangle), id);
                    obj.set_material(self.material);
                    Intersection::new_uv(t, obj, u, v)
                }).collect();
                intersections(v)
            },
            CsgGroup(csg) => csg.intersect(&ray)
        };
    }

    pub fn bounds(&self) -> Bounds {
        return match &self.object_type {
            ObjectShape(shape) => shape.bounds().transform(&self.transformation),
            ObjectGroup(group) => group.bounds(),
            TriangleGroup(model) => model.bounds(),
            SmoothTriangleGroup(model) => model.bounds(),
            CsgGroup(csg) => csg.bounds()
        };
    }
    pub(crate) fn includes(&self, other_obj: &Object) -> bool {
        match &self.object_type {
            ObjectShape(_) => self.object_id == other_obj.object_id,
            ObjectGroup(object_group) => object_group.includes(other_obj),
            TriangleGroup(triangle_group) => triangle_group.includes(other_obj),
            SmoothTriangleGroup(smooth_triangle_group) => smooth_triangle_group.includes(other_obj),
            CsgGroup(csg) => csg.includes(other_obj)
        }
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
            TriangleGroup(model) => {
                model.set_transformation(transformation);
            }
            SmoothTriangleGroup(model) => model.set_transformation(transformation),
            CsgGroup(csg) => csg.set_transformation(transformation),
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
    pub fn material(&self) -> &Material { &(self.material) }
    pub fn set_material(&mut self, material: Material) -> &Self {
        self.material = material;
        self
    }

    pub fn new(shape: Shape) -> Object {
        Object {
            object_id: get_next_unique_shape_id(),
            object_type: ObjectShape(shape),
            material: Material::new(),
            transformation: Matrix::<4>::identity(),
            transformation_inverse: Matrix::<4>::identity(),
            transformation_inverse_transpose: Matrix::<4>::identity(),
        }
    }

    pub fn new_with_id(shape: Shape, id : usize) -> Object {
        Object {
            object_id: id,
            object_type: ObjectShape(shape),
            material: Material::new(),
            transformation: Matrix::<4>::identity(),
            transformation_inverse: Matrix::<4>::identity(),
            transformation_inverse_transpose: Matrix::<4>::identity(),
        }
    }

    pub fn new_group(group: Group) -> Object {
        Object {
            object_id: get_next_unique_shape_id(),
            object_type: ObjectGroup(group),
            material: Material::new(),
            transformation: Matrix::<4>::identity(),
            transformation_inverse: Matrix::<4>::identity(),
            transformation_inverse_transpose: Matrix::<4>::identity(),
        }
    }

    pub fn new_triangle_group(model: TriangleModel) -> Object {
        Object {
            object_id: get_next_unique_shape_id(),
            object_type: TriangleGroup(model),
            material: Material::new(),
            transformation: Matrix::<4>::identity(),
            transformation_inverse: Matrix::<4>::identity(),
            transformation_inverse_transpose: Matrix::<4>::identity(),
        }
    }

    pub fn new_smooth_triangle_group(model: SmoothTriangleModel) -> Object {
        Object {
            object_id: get_next_unique_shape_id(),
            object_type: SmoothTriangleGroup(model),
            material: Material::new(),
            transformation: Matrix::<4>::identity(),
            transformation_inverse: Matrix::<4>::identity(),
            transformation_inverse_transpose: Matrix::<4>::identity(),
        }
    }

    pub fn new_csg(csg: Csg) -> Object {
        Object {
            object_id: get_next_unique_shape_id(),
            object_type: CsgGroup(csg),
            material: Material::new(),
            transformation: Matrix::<4>::identity(),
            transformation_inverse: Matrix::<4>::identity(),
            transformation_inverse_transpose: Matrix::<4>::identity(),
        }
    }

    pub fn get_child_ids(&self) -> Vec<usize>{
        match &self.object_type {
            ObjectShape(_) => vec![],
            ObjectGroup(group) => group.get_child_ids(),
            TriangleGroup(triangle_group) => triangle_group.get_child_ids(),
            SmoothTriangleGroup(smooth_triangle_group) => smooth_triangle_group.get_child_ids(),
            CsgGroup(csg) => csg.get_child_ids()
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
