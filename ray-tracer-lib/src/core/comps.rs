use crate::core::intersection::Intersection;
use crate::core::intersections::Intersections;
use crate::core::math::{EPSILON, Float};
use crate::object::Object;
use crate::core::ray::Ray;
use crate::core::tuple::{Tuple};

pub struct Comps {
    pub object: Object,
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

impl Comps {
    pub fn schlick(&self) -> Float {
        // find the cosine of the angle between the eye and normal vectors
        let mut cos = self.eyev.dot(&self.normalv);
        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n * n * (1.0 - cos * cos);
            if sin2_t > 1.0 {
                return 1.0;
            }
            // compute cosine of theta_t using trig identity
            let cos_t = (1.0 - sin2_t).sqrt();

            // when n1 > n2, use cos(theta_t) instead
            cos = cos_t;
        }

        let mut r0 = (self.n1 - self.n2) / (self.n1 + self.n2);
        r0 *= r0;
        return r0 + (1.0 - r0) * (1.0 - cos).powi(5);
    }
}

pub fn prepare_computations(hit: &Intersection, ray: &Ray, xs: &Intersections) -> Comps {
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

    let mut containers: Vec<Object> = vec!();
    let mut n1 = 0.0;
    let mut n2 = 0.0;

    for intersection in xs.intersections.iter() {
        let x = intersection == hit;
        if x {
            n1 = get_refractive_index(&containers);
        }

        let position = containers.iter().position(|object| { object == &intersection.object });
        match position {
            None => { containers.push(intersection.object.clone()); }
            Some(index) => { containers.remove(index); }
        }

        if x {
            n2 = get_refractive_index(&containers);
            break;
        }
    }

    Comps { t: hit.t, object: hit.object.clone(), point, eyev, normalv, inside, over_point, under_point, reflectv, n1, n2 }
}

fn get_refractive_index(containers: &Vec<Object>) -> Float {
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
