use std::mem;
use crate::bounds::Bounds;
use crate::math;
use crate::math::{EPSILON, Float};
use crate::ray::Ray;
use crate::tuple::{point, Tuple, vector};

#[derive(Debug, Copy, Clone)]
pub struct Cube {}

impl Cube {
    pub(crate) fn normal_at(&self, point: Tuple) -> Tuple {
        let max_c = math::max(point.x.abs(), point.y.abs(), point.z.abs());
        if max_c == point.x.abs() {
            return vector(point.x, 0.0, 0.0);
        } else if max_c == point.y.abs() {
            return vector(0.0, point.y, 0.0);
        }
        vector(0.0, 0.0, point.z)
    }
}

impl Cube {
    pub fn new() -> Self {
        Cube {}
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<Float> {
        let (x_t_min, x_t_max) = self.check_axis(ray.origin.x, ray.direction.x);
        let (y_t_min, y_t_max) = self.check_axis(ray.origin.y, ray.direction.y);
        let (z_t_min, z_t_max) = self.check_axis(ray.origin.z, ray.direction.z);
        let t_min = math::max(x_t_min, y_t_min, z_t_min);
        let t_max = math::min(x_t_max, y_t_max, z_t_max);
        if t_min > t_max {
            return vec![];
        }

        vec![t_min, t_max]
    }

    pub fn check_axis(&self, origin: Float, direction: Float) -> (Float, Float) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;
        let mut tmin: Float;
        let mut tmax: Float;

        if direction.abs() >= EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * math::INFINITY;
            tmax = tmax_numerator * math::INFINITY;
        }

        if tmin > tmax {
            mem::swap(&mut tmin, &mut tmax);
        }
        return (tmin, tmax);
    }

    pub(crate) fn bounds(&self) -> Bounds {
        Bounds::from( point(-1.0, -1.0, -1.0),  point(1.0, 1.0, 1.0))
    }
}
