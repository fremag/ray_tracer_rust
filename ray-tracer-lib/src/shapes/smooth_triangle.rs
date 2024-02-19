use crate::core::bounds::Bounds;
use crate::core::intersection::Intersection;
use crate::core::math::Float;
use crate::core::ray::Ray;
use crate::core::tuple::Tuple;
use crate::shapes::triangle::Triangle;

#[derive(Debug, Clone)]
pub struct SmoothTriangle {
    pub triangle : Triangle,
    pub n1: Tuple,
    pub n2: Tuple,
    pub n3: Tuple,
}

impl SmoothTriangle {
    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<Float> {
        self.triangle.intersect(ray)
    }

    pub(crate) fn normal_at(&self, _point: &Tuple, intersection: &Intersection) -> Tuple {
        let a = self.n2 * intersection.u;
        let b=  self.n3 * intersection.v;
        let c = self.n1 * (1.0 - intersection.u - intersection.v);
        let n = a+b+c;
        n.normalize()
    }

    pub(crate) fn intersect_uv(&self, ray: &Ray) -> (Float, Float, Float) {
        self.triangle.intersect_uv(ray)
    }

    pub fn new(p1 : Tuple, p2: Tuple, p3 : Tuple, n1 : Tuple,  n2 : Tuple, n3 : Tuple) -> Self {
        Self { triangle: Triangle::new(p1, p2, p3), n1, n2, n3}
    }

    pub fn bounds(&self) -> Bounds {
        self.triangle.bounds()
    }
}

impl PartialEq for SmoothTriangle {
    fn eq(&self, other: &Self) -> bool {
        self.triangle == other.triangle && self.n1 == other.n1 && self.n2 == other.n2 && self.n3 == other.n3
    }
}
