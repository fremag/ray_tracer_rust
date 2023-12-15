use crate::colors::Color;
use crate::comps::{Comps, prepare_computations};
use crate::intersections::{Intersections, intersections};
use crate::light::Light;
use crate::light::PointLight;
use crate::material::{Material};
use crate::object::{build_sphere, Object};
use crate::ray::Ray;
use crate::transform::scaling;
use crate::tuple::point;

pub struct World {
    pub objects : Vec<Object>,
    pub lights : Vec<Light>
}

impl World {
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

    pub fn shade_hit(&self, comps : &Comps) -> Color {
        let mut c= Color::new(0.0, 0.0, 0.0);
        let material = comps.object.material();

        for light in self.lights.iter() {
            c = c + material.lighting(&light, comps.point, comps.eyev, comps.normalv);
        }

        c
    }

    pub fn set_lights(&mut self, lights : Vec<Light>) {
        self.lights = lights;
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect_world(ray);
        let hit = intersections.hit();
        match hit {
            None => Color::new(0.0, 0.0, 0.0),
            Some(intersection) => {
                let comps = prepare_computations(intersection, ray);
                self.shade_hit(&comps)
            }
        }
    }
}

pub fn build_world() -> World {
    let mut sphere_1 = build_sphere();
    sphere_1.set_material(Material {color: Color::new(0.8, 1.0, 0.6),
    diffuse: 0.7, specular: 0.2, shininess: 200.0, ambient: 0.1});

    let mut sphere_2 = build_sphere();
    sphere_2.set_transformation(scaling(0.5, 0.5, 0.5));

    let light = PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    World {objects: vec!(sphere_1, sphere_2), lights: vec!(light) }
}
