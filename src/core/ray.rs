use crate::core::math::Float;
use crate::core::matrix::Matrix;
use crate::core::tuple::Tuple;

pub struct Ray  {
    pub origin: Tuple,
    pub direction : Tuple
}

impl Ray {
    pub fn position(&self, t: Float) -> Tuple {
        self.origin + t * self.direction
    }
    pub fn transform(&self, transformation: &Matrix<4>) -> Ray {
        Ray {
            origin: transformation * &self.origin,
            direction: transformation * &self.direction,
        }
    }
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray {origin, direction}
}
