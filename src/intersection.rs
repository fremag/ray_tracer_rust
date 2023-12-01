use std::cmp::Ordering;
use crate::math::Float;
use crate::shape::{Shape};

#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    pub t : Float,
    pub shape: Shape
}

impl Intersection {
    pub fn new(t : Float, shape : Shape) -> Self {
        let inter = Intersection {t, shape};
        inter
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        if self.t != other.t {
            return false;
        }

        self.shape == other.shape
    }
}

impl Eq for Intersection {

}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t.is_nan() {
            Ordering::Greater
        } else if other.t.is_nan() {
            Ordering::Less
        } else if self.t > other.t {
            Ordering::Greater
        } else if self.t < other.t {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}