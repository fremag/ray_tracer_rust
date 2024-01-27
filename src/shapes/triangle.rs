use crate::core::bounds::Bounds;
use crate::core::math::Float;
use crate::core::ray::Ray;
use crate::core::tuple::Tuple;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    pub e1: Tuple,
    pub e2: Tuple,
    pub normal: Tuple,
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.p1 == other.p1 && self.p2 == other.p2 && self.p3 == other.p3
    }
}

impl Triangle {
    pub fn new(p1 : Tuple, p2 : Tuple, p3 : Tuple) -> Self {
        let e1 = p2-p1;
        let e2 = p3-p1;
        let normal = e2 * &e1;
        Self {p1, p2, p3, e1, e2, normal}
    }

    pub(crate) fn intersect(&self, point: &Ray) -> Vec<Float> {
        todo!()
    }

    pub(crate) fn normal_at(&self, _: &Tuple) -> Tuple {
        self.normal
    }

    pub(crate) fn bounds(&self) -> Bounds {
        todo!()
    }
}