use crate::core::bounds::Bounds;
use crate::core::math;
use crate::core::math::{Float};
use crate::core::ray::Ray;
use crate::core::tuple::{point, Tuple, vector};

#[derive(Debug, Copy, Clone)]
pub struct Cube {}

impl Cube {
    pub fn normal_at(&self, point: Tuple) -> Tuple {
        let max_c = math::max(point.x.abs(), point.y.abs(), point.z.abs());
        if max_c == point.x.abs() {
            return vector(point.x, 0.0, 0.0);
        } else if max_c == point.y.abs() {
            return vector(0.0, point.y, 0.0);
        }
        vector(0.0, 0.0, point.z)
    }

    pub fn new() -> Self {
        Cube {}
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Float> {
        self.bounds().intersect(&ray)
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::from( point(-1.0, -1.0, -1.0),  point(1.0, 1.0, 1.0))
    }
}
