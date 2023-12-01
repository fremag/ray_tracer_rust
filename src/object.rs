use crate::intersections::Intersections;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::sphere;

pub struct Object {
    shape : Shape,
    transformation: Matrix<4>,
    transformation_inverse: Matrix<4> // optimization: keep inverse transformation
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
    }
}