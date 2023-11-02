use crate::math::{equals, Float};


#[derive(Debug)]
pub struct Tuple {
    pub(crate) x : Float,
    pub(crate) y : Float,
    pub(crate) z : Float,
    pub(crate) w : Float
}

impl Tuple {
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        equals(self.x, other.x) &&
            equals(self.y, other.y) &&
            equals(self.z, other.z) &&
            equals(self.w, other.w)
    }
}
pub fn point(x : Float, y : Float, z : Float) -> Tuple {
    Tuple { x, y, z, w : 1.0}
}
pub fn vector(x : Float, y : Float, z : Float) -> Tuple {
    Tuple { x, y, z, w : 0.0}
}
