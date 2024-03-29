use crate::core::bounds::Bounds;
use crate::core::math;
use crate::core::math::Float;
use crate::core::ray::Ray;
use crate::core::tuple::Tuple;
use crate::object::get_next_unique_shape_id;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub id : usize,
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
        let id = get_next_unique_shape_id();
        let e1 = p2-p1;
        let e2 = p3-p1;
        let normal = (e2 * &e1).normalize();
        Self {id, p1, p2, p3, e1, e2, normal}
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Float> {
        let (t, _u, _v) = self.intersect_uv(ray);
        if Float::is_nan(t) {
            return vec![]
        }
        vec![t]
    }

    pub fn intersect_uv(&self, ray: &Ray) -> (Float, Float, Float) {
        let dir_cross_e2 = ray.direction * &self.e2;
        let det = self.e1.dot(&dir_cross_e2);
        if det.abs() < math::EPSILON {
            return (Float::NAN, Float::NAN, Float::NAN);
        }
        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * p1_to_origin.dot(&dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return (Float::NAN, Float::NAN, Float::NAN);
        }

        let origin_cross_e1 = p1_to_origin * &self.e1;
        let v = f * ray.direction.dot(&origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return (Float::NAN, Float::NAN, Float::NAN);
        }

        let t = f * self.e2.dot(&origin_cross_e1);
        (t, u, v)
    }

    pub fn normal_at(&self, _: &Tuple) -> Tuple {
        self.normal
    }

    pub fn bounds(&self) -> Bounds {
        let mut bounds = Bounds::new();
        bounds.add(&self.p1);
        bounds.add(&self.p2);
        bounds.add(&self.p3);

        bounds
    }
}