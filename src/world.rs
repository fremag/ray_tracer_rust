use crate::colors::Color;
use crate::core::comps::{Comps, prepare_computations};
use crate::core::intersections::{Intersections, intersections};
use crate::lights::light::Light;
use crate::lights::point_light::PointLight;
use crate::material::{Material};
use crate::object::{build_sphere, Object};
use crate::patterns::pattern::{Pattern};
use crate::core::ray::{Ray, ray};
use crate::core::transform::scaling;
use crate::core::tuple::{point, Tuple};

pub struct World {
    pub objects : Vec<Object>,
    pub lights : Vec<Light>
}

impl<'a> World {

    pub(crate) fn new() -> World {
        World {objects: vec![], lights: vec!()}
    }

    pub(crate) fn intersect_world(&self, ray: &Ray) -> Intersections
    {
        let mut intersection_vec= vec![];
        for object in self.objects.iter() {
            let obj_intersections = object.intersect(ray);
            for intersection in obj_intersections.intersections.iter() {
                intersection_vec.push(*intersection)
            }
        }

        intersections(intersection_vec)
    }

    pub fn shade_hit(&self, comps : &Comps, remaining: i32) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let material = comps.object.material();

        for light in self.lights.iter() {
            let in_shadow = self.is_shadowed(light, comps.over_point);
            let surface = material.lighting(comps.object, &light, comps.over_point, comps.eyev, comps.normalv, in_shadow);
            let reflected= self.reflected_color(comps, remaining);
            let refracted= self.refracted_color(comps, remaining);

            if material.reflective > 0.0 && material.transparency > 0.0 {
                let reflectance = comps.schlick();
                color = color + surface + reflected * reflectance + refracted * (1.0 - reflectance);
            } else {
                color = color + surface + reflected + refracted;
            }
        }

        color
    }

    pub fn set_lights(&mut self, lights : Vec<Light>) {
        self.lights = lights;
    }

    pub fn set_objects(&mut self, objects : Vec<Object>) {
        self.objects = objects;
    }

    pub fn color_at(&self, ray: &Ray, remaining : i32) -> Color {
        let intersections = self.intersect_world(ray);
        let hit = intersections.hit();
        match hit {
            None => Color::new(0.0, 0.0, 0.0),
            Some(intersection) => {
                let comps = prepare_computations(intersection, ray, &intersections);
                self.shade_hit(&comps, remaining)
            }
        }
    }

    pub(crate) fn is_shadowed(&self, light: &Light, point: Tuple) -> bool {
        let v = light.position() - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = ray(point, direction);
        let intersections = self.intersect_world(&r);
        let h = intersections.hit();
        match h {
            None => false,
            Some(intersection) =>
                {
                    let t = intersection.t;
                    let shadow = t < distance;
                    shadow
                }
        }
    }

    pub(crate) fn reflected_color(&self, comps: &Comps, remaining: i32) -> Color {
        if comps.object.material().reflective == 0.0 || remaining == 0 {
            return Color::black();
        }

        let reflect_ray = ray(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray, remaining-1);
        return color * comps.object.material().reflective;
    }

    pub(crate) fn refracted_color(&self, comps: &Comps, remaining: i32) -> Color {
        if comps.object.material().transparency == 0.0 || remaining == 0 {
            return Color::black();
        }

        let n_ratio = comps.n1 / comps.n2;
        // cos(theta_i) is the same as the dot product of the two vectors
        let cos_i = comps.eyev.dot(&comps.normalv);
        // Find sin(theta_t)^2 via trigonometric identity
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);
        if sin2_t > 1.0 {
            return Color::black();
        }

        // Find cos(theta_t) via trigonometric identity
        let cos_t = (1.0 - sin2_t).sqrt();
        // Compute the direction of the refracted ray
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        // Create the refracted ray
        let refract_ray = ray(comps.under_point, direction);

        // Find the color of the refracted ray, making sure to multiply
        // by the transparency value to account for any opacity
        let color = self.color_at(&refract_ray, remaining - 1) * comps.object.material().transparency;
        return color;
    }
}

pub fn build_world() -> World {
    let mut sphere_1 = build_sphere();
    sphere_1.set_material(Material {color: Color::new(0.8, 1.0, 0.6),
    diffuse: 0.7, specular: 0.2, shininess: 200.0, ambient: 0.1, reflective: 0.0,
        transparency: 0.0, refractive_index: 1.0, pattern: Pattern::new()});

    let mut sphere_2 = build_sphere();
    sphere_2.set_transformation(scaling(0.5, 0.5, 0.5));

    let light = PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    World {objects: vec!(sphere_1, sphere_2), lights: vec!(light) }
}

