use crate::canvas::Canvas;
use crate::colors::Color;

#[test]
fn creating_a_canvas_test() {
    let canvas = Canvas::new(10, 20);
    assert_eq!(canvas.width, 10);
    assert_eq!(canvas.height, 20);
    let black = Color::new(0.0, 0.0, 0.0);

    for y in 0..canvas.height {
        for x in 0..canvas.width {
            assert_eq!(canvas.pixel_at(x, y), black)
        }
    }
}

#[test]
fn writing_pixels_to_a_canvas_test() {
    let mut canvas = Canvas::new(10, 20);
    let red= Color::new(1.0, 0.0, 0.0);
    canvas.write_pixel(2, 3, red);
    let pixel_color = canvas.pixel_at(2, 3);
    assert_eq!(pixel_color, red)
}