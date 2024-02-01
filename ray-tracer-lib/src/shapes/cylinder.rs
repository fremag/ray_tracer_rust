use std::mem;
use crate::core::bounds::Bounds;
use crate::core::math::{EPSILON, equals, Float, INFINITY};
use crate::core::ray::Ray;
use crate::core::tuple::{point, Tuple, vector};

#[derive(Debug, Copy, Clone)]
pub struct Cylinder {
    pub min: Float,
    pub max: Float,
    pub closed : bool,
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max && self.closed == other.closed
    }
}

impl Eq for Cylinder {
}

impl Cylinder {
    pub fn new() -> Self {
        Cylinder {min: -INFINITY, max: INFINITY, closed: false}
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Float> {
        let mut xs = vec![];

        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        //  ray is parallel to the y-axis
        if ! equals(a, 0.0) {
            let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
            let c = ray.origin.x * ray.origin.x + ray.origin.z * ray.origin.z - 1.0;
            let disc = b * b - 4.0 * a * c;
            // ray does not intersect the cylinder
            if disc < 0.0 {
                return vec![];
            }

            let inv_2a = 1.0 / (2.0 * a);
            let sqrt_disc = disc.sqrt();
            let mut t0 = (-b - sqrt_disc) * inv_2a;
            let mut t1 = (-b + sqrt_disc) * inv_2a;

            if t0 > t1 {
                mem::swap(&mut t0, &mut t1);
            }

            let y0 = ray.origin.y + t0 * ray.direction.y;
            if self.min < y0 && y0 < self.max {
                xs.push(t0);
            }

            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.min < y1 && y1 < self.max {
                xs.push(t1);
            }
        }

        self.intersect_caps(ray, &mut xs);

        xs
    }

    pub fn from(min : Float, max : Float, closed : bool) -> Self {
        Cylinder {min, max, closed}
    }

    pub fn normal_at(&self, point: &Tuple) -> Tuple {
        // compute the square of the distance from the y axis
        let dist = point.x * point.x + point.z * point.z;
        if dist < 1.0 && point.y >= self.max - EPSILON {
            return vector(0.0, 1.0, 0.0);
        } else if dist < 1.0 && point.y <= self.min + EPSILON {
            return vector(0.0, -1.0, 0.0);
        }

        vector(point.x, 0.0, point.z)
    }

    // a helper function to reduce duplication.
    // checks to see if the intersection at `t` is within a radius
    // of 1 (the radius of your cylinders) from the y axis.
    fn check_cap(&self, ray : &Ray, t : Float) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        x * x + z * z <= 1.0
    }

    fn intersect_caps(&self, ray : &Ray, xs : &mut Vec<Float>) {
        // caps only matter if the cylinder is closed, and might possibly be
        // intersected by the ray.
        if !self.closed || equals(ray.direction.y, 0.0) {
            return;
        }

        // check for an intersection with the lower end cap by intersecting
        // the ray with the plane at y=cyl.minimum
        let t = (self.min - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, t) {
            xs.push(t);
        }

        // check for an intersection with the upper end cap by intersecting
        // the ray with the plane at y=cyl.maximum
        let t = (self.max - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, t) {
            xs.push(t);
        }
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::from( point(-1.0, self.min, -1.0),  point(1.0, self.max, 1.0))
    }
}
