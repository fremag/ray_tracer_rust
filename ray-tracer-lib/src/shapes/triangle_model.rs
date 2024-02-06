use crate::core::bounds::Bounds;
use crate::core::intersections::Intersections;
use crate::core::matrix::Matrix;
use crate::core::ray::Ray;
use crate::shapes::triangle::Triangle;

#[derive(Debug, Clone)]
pub struct TriangleModel {
    pub triangles : Vec<Triangle>
}

impl TriangleModel {
    pub fn new(triangles : Vec<Triangle>) -> Self {
        Self {triangles}
    }

    pub(crate) fn set_transformation(&self, transformation: Matrix<4>) {
        todo!()
    }

    pub(crate) fn bounds(&self) -> Bounds {
        todo!()
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Intersections {
        todo!()
    }
}