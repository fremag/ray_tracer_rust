#[cfg(test)]
mod tests {
    use crate::colors::Color;
    use crate::lights::light::Light;
    use crate::lights::point_light::PointLight;
    use crate::core::tuple::point;

    #[test]
    fn a_point_light_has_a_position_and_intensity_test() {
        let intensity = Color { r: 1.0, g: 1.0, b: 1.0 };
        let position = point(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        match light {
            Light::PointLight(point_light) => {
                assert_eq!(point_light.position, position);
                assert_eq!(point_light.intensity, intensity);
            }
        }
    }
}