use std::mem;
use crate::core::bounds::Bounds;
use crate::core::math::{EPSILON, equals, Float};
use crate::core::ray::Ray;
use crate::core::tuple::{point, Tuple, vector};

#[derive(Debug, Copy, Clone)]
pub struct Cone {
    pub min: Float,
    pub max: Float,
    pub closed : bool,
}

impl PartialEq for Cone {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max && self.closed == other.closed
    }
}

impl Eq for Cone {
}

impl Cone {
    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<Float> {
        let mut xs = vec![];

        let a = ray.direction.x * ray.direction.x - ray.direction.y * ray.direction.y + ray.direction.z * ray.direction.z;
        let b = 2.0 * (ray.origin.x * ray.direction.x - ray.origin.y * ray.direction.y +  ray.origin.z * ray.direction.z);
        let c = ray.origin.x * ray.origin.x - ray.origin.y * ray.origin.y + ray.origin.z * ray.origin.z;

        //  ray is not parallel to the cone
        if equals(a, 0.0) {
            let t = -c / (2.0 * b);
            xs.push(t);
        } else {
            let disc = b * b - 4.0 * a * c;
            // ray does not intersect the cone
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

    pub(crate) fn from(min : Float, max : Float, closed : bool) -> Self {
        Cone {min, max, closed}
    }

    pub(crate) fn normal_at(&self, point: &Tuple) -> Tuple {
        // compute the square of the distance from the y axis
        let dist = point.x * point.x + point.z * point.z;
        if dist < 1.0 && point.y >= self.max - EPSILON {
            return vector(0.0, 1.0, 0.0);
        } else if dist < 1.0 && point.y <= self.min + EPSILON {
            return vector(0.0, -1.0, 0.0);
        }

        let mut y = (point.x * point.x + point.z * point.z).sqrt();
        if point.y > 0.0 {
            y = -y;
        }
        vector(point.x, y, point.z)
    }

    // a helper function to reduce duplication.
    // checks to see if the intersection at `t` is within a radius
    // of 1 (the radius of your cone) from the y axis.
    fn check_cap(&self, ray : &Ray, t : Float, y : Float) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        x * x + z * z <= y*y
    }

    fn intersect_caps(&self, ray : &Ray, xs : &mut Vec<Float>) {
        // caps only matter if the cone is closed, and might possibly be
        // intersected by the ray.
        if !self.closed || equals(ray.direction.y, 0.0) {
            return;
        }

        // check for an intersection with the lower end cap by intersecting
        // the ray with the plane at y=cone.minimum
        let t = (self.min - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, t, self.min) {
            xs.push(t);
        }

        // check for an intersection with the upper end cap by intersecting
        // the ray with the plane at y=cone.maximum
        let t = (self.max - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, t, self.max) {
            xs.push(t);
        }
    }

    pub(crate) fn bounds(&self) -> Bounds {
        Bounds::from( point(self.min, self.min, self.min),  point(self.max, self.max, self.max))
    }
}
