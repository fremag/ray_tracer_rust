use crate::core::bounds::Bounds;
use crate::core::math::{EPSILON, Float};
use crate::core::ray::Ray;
use crate::core::tuple::{point, Tuple, vector};

#[derive(Debug, Clone, Copy)]
pub struct Plane {

}

impl Plane {
    pub fn new() -> Plane {
        Plane { }
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<Float> {
        if (ray.direction.y.abs()) < EPSILON {
            return vec![]; // empty set - -no intersections
        }

        let t = -ray.origin.y / ray.direction.y;
        return vec![t];
    }

    pub(crate) fn normal_at(&self, _ : Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
    }

    pub(crate) fn bounds(&self) -> Bounds {
        Bounds::from(point(-Float::INFINITY, 0.0, -Float::INFINITY), point(Float::INFINITY, 0.0, Float::INFINITY))
    }
}