use crate::intersections::Intersections;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::sphere;
use crate::tuple::Tuple;

pub struct Object {
    shape : Shape,
    transformation: Matrix<4>,
    transformation_inverse: Matrix<4>, // optimization: keep inverse transformation
    transformation_inverse_transpose: Matrix<4>, // optimization: keep inverse transformation transpose
}

impl Object {
    pub(crate) fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = &self.transformation_inverse * &world_point;
        let object_normal = self.shape.normal_at(object_point);
        let mut world_normal = &self.transformation_inverse_transpose * &object_normal;
        world_normal.w = 0.0;
        let n = world_normal.normalize();
        n
    }
}

impl Object {
    pub(crate) fn intersect(&self, ray: Ray) -> Intersections {
        let ray2 = ray.transform( &self.transformation_inverse);
        self.shape.intersect(&ray2)
    }
}

impl Object {
    pub fn set_transformation(&mut self, transformation : Matrix<4> ) -> &Self {
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
    pub fn shape(&self) -> Shape { self.shape }
}

pub fn build_sphere() -> Object {
    Object {
        shape: sphere(),
        transformation: Matrix::<4>::identity(),
        transformation_inverse: Matrix::<4>::identity(),
        transformation_inverse_transpose: Matrix::<4>::identity(),
    }
}