use crate::math::Float;
use crate::ray::Ray;
use crate::tuple::point;

pub struct Sphere {

}

impl Sphere {
    pub(crate) fn intersect(&self, ray : &Ray) -> Vec<Float> {
        // the vector from the sphere's center, to the ray origin
        // remember: the sphere is centered at the world origin
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);
        let a = ray.direction.dot( &ray.direction);
        let b = 2.0 * ray.direction.dot( &sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec!()
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        vec! (t1, t2)
    }
}

pub fn sphere() -> Sphere {
    Sphere { }
}
