use crate::math::{equals, Float};
use std::ops::{Add, Sub, Neg, Mul, Div, Index};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub(crate) x : Float,
    pub(crate) y : Float,
    pub(crate) z : Float,
    pub(crate) w : Float
}

impl Tuple {
    pub fn new(x: Float, y : Float, z : Float, w: Float) -> Self {
        Tuple {x, y, z, w }
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn magnitude(&self) -> Float {
        (self.x*self.x+self.y*self.y+self.z*self.z).sqrt()
    }
    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        Tuple::new(self.x / mag, self.y / mag, self.z / mag, self.w / mag)
    }

    pub fn dot(&self, v : &Tuple) -> Float {
        self.x * v.x + self.y * v.y +self.z * v.z
    }

    pub fn reflect(&self, normal : &Tuple) -> Tuple {
        let c = 2.0 * self.dot(normal);
        let n = *normal * c;
        let reflected = self - &n;
        reflected
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

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w}
    }
}
impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w}
    }
}

impl Sub for &Tuple {
    type Output = Tuple;

    fn sub(self, &rhs: Self) -> Self::Output {
        Tuple {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w}
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {x: -self.x, y: -self.y, z: -self.z, w: -self.w}
    }
}

impl Mul<Float> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Float) -> Self::Output {
        Tuple{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs}
    }
}

impl Mul<Tuple> for Float {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Tuple> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Self::Output {
        vector(self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x)
    }
}

impl Div<Float> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: Float) -> Self::Output {
        Tuple{ x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs}
    }
}
pub fn point(x : Float, y : Float, z : Float) -> Tuple {
    Tuple { x, y, z, w : 1.0}
}
pub fn vector(x : Float, y : Float, z : Float) -> Tuple {
    Tuple { x, y, z, w : 0.0}
}

impl Index<usize> for Tuple {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x, 1 => &self.y, 2 => &self.z, 3 => &self.w,
            _ => &f64::NAN as &Float
        }
    }
}
