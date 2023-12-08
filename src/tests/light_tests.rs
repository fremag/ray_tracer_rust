#[cfg(test)]
mod light_test {
    use crate::colors::Color;
    use crate::light::Light;
    use crate::tuple::point;

    #[test]
    fn a_point_light_has_a_position_and_intensity_test() {
        let intensity = Color {r: 1.0, g: 1.0,b:  1.0};
        let position = point(0.0, 0.0, 0.0);
        let light = Light {position, intensity };
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}