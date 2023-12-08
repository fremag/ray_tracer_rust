#[cfg(test)]
mod material_tests {
    use crate::colors::Color;
    use crate::material::material;

    #[test]
    fn default_material_test() {
        let default_material = material();
        assert_eq!(default_material.color, Color {r: 1.0, g: 1.0, b: 1.0});
        assert_eq!(default_material.ambient, 0.1);
        assert_eq!(default_material.diffuse, 0.9);
        assert_eq!(default_material.specular, 0.9);
        assert_eq!(default_material.shininess, 200.0);
    }
}