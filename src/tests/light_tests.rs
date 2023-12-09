#[cfg(test)]
mod light_test {
    use crate::colors::Color;
    use crate::light::{Light, PointLight};
    use crate::tuple::point;

    #[test]
    fn a_point_light_has_a_position_and_intensity_test() {
        let intensity = Color {r: 1.0, g: 1.0,b:  1.0};
        let position = point(0.0, 0.0, 0.0);
        let Light::PointLight(point_light) = PointLight::new(position, intensity );
        assert_eq!(point_light.position, position);
        assert_eq!(point_light.intensity, intensity);
    }
}