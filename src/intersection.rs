use std::cmp::Ordering;
use crate::math::Float;
use crate::object::Object;

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    pub t : Float,
    pub object: &'a Object<'a>
}

impl<'a> Intersection<'a> {
    pub fn new(t : Float, object : &'a Object) -> Self {
        let inter = Intersection {t, object};
        inter
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.t != other.t {
            return false;
        }

        self.object.shape() == other.object.shape()
    }
}

impl<'a> Eq for Intersection<'a> {

}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Intersection<'a> {
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