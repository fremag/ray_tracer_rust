#[cfg(test)]
mod material_tests {
    use crate::colors::Color;
    use crate::light::{Light, PointLight};
    use crate::material::material;
    use crate::math::SQRT2;
    use crate::tuple::{point, vector};

    #[test]
    fn default_material_test() {
        let default_material = material();
        assert_eq!(default_material.color, Color { r: 1.0, g: 1.0, b: 1.0 });
        assert_eq!(default_material.ambient, 0.1);
        assert_eq!(default_material.diffuse, 0.9);
        assert_eq!(default_material.specular, 0.9);
        assert_eq!(default_material.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_test() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light: Light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees_test() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, SQRT2 / 2.0, -SQRT2 / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees_test() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector_test() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, -SQRT2 / 2.0, -SQRT2 / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface_test() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}