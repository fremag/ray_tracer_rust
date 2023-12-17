use crate::intersection::Intersection;
use crate::math::{EPSILON, Float};
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;

pub struct Comps<'a> {
    pub object : &'a Object,
    pub t : Float,
    pub point : Tuple,
    pub eyev : Tuple,
    pub normalv : Tuple,
    pub inside : bool,
    pub over_point : Tuple,
}

pub fn prepare_computations<'a>(intersection: &'a Intersection, ray : &Ray) -> Comps<'a> {
    // instantiate a data structure for storing some precomputed values
    let point = ray.position(intersection.t);
    let eyev = -ray.direction;
    let mut normalv = intersection.object.normal_at(point);
    let mut inside = false;

    if normalv.dot(&eyev) < 0.0 {
        inside = true;
        normalv = -normalv;
    }
    let over_point= point + normalv * EPSILON;
    Comps {t: intersection.t, object: intersection.object, point, eyev , normalv, inside, over_point}
}
