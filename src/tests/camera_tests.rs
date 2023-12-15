#[cfg(test)]
mod camera_tests {
    use crate::camera::Camera;
    use crate::math::PI;
    use crate::matrix::Matrix;

    #[test]
    fn constructing_a_camera_test() {
        let h_size = 160;
        let v_size = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(h_size, v_size, field_of_view);
        assert_eq!(c.h_size, 160);
        assert_eq!(c.v_size, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, Matrix::<4>::identity());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas_test() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas_test() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert_eq!(c.pixel_size, 0.01);
    }
}