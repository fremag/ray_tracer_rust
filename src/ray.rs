use crate::math::Float;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub struct Ray  {
    pub(crate) origin: Tuple,
    pub(crate) direction : Tuple
}

impl Ray {
    pub(crate) fn position(&self, t: Float) -> Tuple {
        self.origin + t * self.direction
    }
    pub(crate) fn transform(&self, transformation: &Matrix<4>) -> Ray {
        Ray {
            origin: transformation * &self.origin,
            direction: transformation * &self.direction,
        }
    }
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray {origin, direction}
}
