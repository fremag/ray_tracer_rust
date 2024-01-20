use crate::math::Float;
use crate::matrix::Matrix;
use crate::tuple::{point, Tuple};

#[derive(Debug, Clone)]
pub struct Bounds {
    pub min : Tuple,
    pub max : Tuple,
}

impl Bounds {
    pub(crate) fn transform(&self, transform: &Matrix<4>) -> Bounds {
        let p1 = self.min;
        let p2 = point(self.min.x, self.min.y, self.max.z);
        let p3 = point(self.min.x, self.max.y, self.min.z);
        let p4 = point(self.min.x, self.max.y, self.max.z);
        let p5 = point(self.max.x, self.min.y, self.min.z);
        let p6 = point(self.max.x, self.min.y, self.max.z);
        let p7 = point(self.max.x, self.max.y, self.min.z);
        let p8 = self.max;

        let mut bounds = Self::from(
            point(Float::INFINITY, Float::INFINITY, Float::INFINITY),
            point(Float::NEG_INFINITY, Float::NEG_INFINITY, Float::NEG_INFINITY)
        );
        bounds.add(&(transform * &p1));
        bounds.add(&(transform * &p2));
        bounds.add(&(transform * &p3));
        bounds.add(&(transform * &p4));
        bounds.add(&(transform * &p5));
        bounds.add(&(transform * &p6));
        bounds.add(&(transform * &p7));
        bounds.add(&(transform * &p8));
        bounds
    }

    pub(crate) fn from(min: Tuple, max: Tuple) -> Bounds {
        Bounds {min, max}
    }

    pub(crate) fn new() -> Bounds {
        Bounds {min: point(0.0, 0.0, 0.0), max: point(0.0, 0.0, 0.0)}
    }

    pub fn add(&mut self, point : &Tuple) {
        self.min.x = Float::min(self.min.x, point.x);
        self.min.y = Float::min(self.min.y, point.y);
        self.min.z = Float::min(self.min.z, point.z);

        self.max.x = Float::max(self.max.x, point.x);
        self.max.y = Float::max(self.max.y, point.y);
        self.max.z = Float::max(self.max.z, point.z);
    }

    pub fn extend(&mut self, bounds: &Bounds) {
        self.min.x = Float::min(self.min.x, bounds.min.x);
        self.min.y = Float::min(self.min.y, bounds.min.y);
        self.min.z = Float::min(self.min.z, bounds.min.z);

        self.max.x = Float::max(self.max.x, bounds.max.x);
        self.max.y = Float::max(self.max.y, bounds.max.y);
        self.max.z = Float::max(self.max.z, bounds.max.z);
    }
}