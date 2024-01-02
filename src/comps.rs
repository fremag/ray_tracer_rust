use crate::intersection::Intersection;
use crate::intersections::Intersections;
use crate::math::{EPSILON, Float};
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::{Tuple};

pub struct Comps<'a> {
    pub object: &'a Object,
    pub t: Float,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub reflectv: Tuple,
    pub n1: Float,
    pub n2: Float,
}

pub fn prepare_computations<'a>(hit: &'a Intersection, ray: &Ray, xs: &Intersections) -> Comps<'a> {
    // instantiate a data structure for storing some precomputed values
    let point = ray.position(hit.t);
    let eyev = -ray.direction;
    let mut normalv = hit.object.normal_at(point);
    let mut inside = false;

    if normalv.dot(&eyev) < 0.0 {
        inside = true;
        normalv = -normalv;
    }

    // after negating the normal, if necessary
    let reflectv = ray.direction.reflect(&normalv);

    let over_point = point + normalv * EPSILON;
    let under_point = point - normalv * EPSILON;

    let mut containers: Vec<&Object> = vec!();
    let mut n1 = 0.0;
    let mut n2 = 0.0;

    for intersection in xs.intersections.iter() {
        let x = intersection == hit;
        if x {
            n1 = get_refractive_index(&containers);
        }

        let position = containers.iter().position(|object| { object == &intersection.object });
        match position {
            None => { containers.push(intersection.object); }
            Some(index) => { containers.remove(index); }
        }

        if x {
            n2 = get_refractive_index(&containers);
            break;
        }
    }

    Comps { t: hit.t, object: hit.object, point, eyev, normalv, inside, over_point, under_point, reflectv, n1, n2 }
}

fn get_refractive_index(containers: &Vec<&Object>) -> Float {
    let n: Float;
    if containers.is_empty() {
        n = 1.0;
    } else {
        n = match containers.last() {
            None => { 1.0 }
            Some(object) => { object.material().refractive_index }
        }
    }

    n
}
