use crate::colors::Color;

#[test]
fn colors_are_rgb_tuples_test() {
    let c = Color{r: -0.5, g : 0.4, b: 1.7};
    assert_eq!(c.r, -0.5);
    assert_eq!(c.g, 0.4);
    assert_eq!(c.b, 1.7);
}

#[test]
fn adding_colors_test() {
    let c1 = Color{r: 0.9, g : 0.6, b: 0.75};
    let c2 = Color{r: 0.7, g : 0.1, b: 0.25};
    let c = c1 + c2;
    let color = Color { r: 1.6, g: 0.7, b: 1.0 };
    assert_eq!(c, color)
}
#[test]
fn subtracting_colors_test() {
    let c1 = Color{r: 0.9, g : 0.6, b: 0.75};
    let c2 = Color{r: 0.7, g : 0.1, b: 0.25};
    let c = c1 - c2;
    let color = Color { r: 0.2, g: 0.5, b: 0.5 };
    assert_eq!(c, color)
}

#[test]
fn multiplying_color_by_scalar_test() {
    let c1 = Color{r: 0.2, g : 0.3, b: 0.4};
    let c = c1 * 2.0;
    let color = Color { r: 0.4, g: 0.6, b: 0.8 };
    assert_eq!(c == color, true)
}
#[test]
fn hadamard_product_colors_test() {
    let c1 = Color{r: 1.0, g : 0.2, b: 0.4};
    let c2 = Color{r: 0.9, g : 1.0, b: 0.1};
    let c = c1 * c2;
    let color = Color { r: 0.9, g: 0.2, b: 0.04 };
    assert_eq!(c == color, true)
}
