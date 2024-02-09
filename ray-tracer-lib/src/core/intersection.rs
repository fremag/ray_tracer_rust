use std::cmp::Ordering;
use crate::core::math::Float;
use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t : Float,
    pub object: Object
}

impl Intersection {
    pub fn new(t : Float, object : Object) -> Self {
        let inter = Intersection {t, object};
        inter
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        if self.t != other.t {
            return false;
        }

        self.object.shape() == other.object.shape()
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