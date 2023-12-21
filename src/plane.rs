use crate::math::{EPSILON, Float};
use crate::ray::Ray;
use crate::tuple::{Tuple, vector};

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
}