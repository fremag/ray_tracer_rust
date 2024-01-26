#[cfg(test)]
pub mod tests {
    use crate::colors::Color;
    use crate::core::transform::scaling;
    use crate::core::tuple::point;
    use crate::lights::point_light::PointLight;
    use crate::material::Material;
    use crate::object::build_sphere;
    use crate::patterns::pattern::Pattern;
    use crate::world::World;

    pub fn build_world() -> World {
        let mut sphere_1 = build_sphere();
        sphere_1.set_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
            ambient: 0.1,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: Pattern::new()
        });

        let mut sphere_2 = build_sphere();
        sphere_2.set_transformation(scaling(0.5, 0.5, 0.5));

        let light = PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        World { objects: vec!(sphere_1, sphere_2), lights: vec!(light) }
    }
}
