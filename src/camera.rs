use crate::math::Float;
use crate::matrix::Matrix;

pub struct Camera {
    pub h_size: usize,
    pub v_size: usize,
    pub field_of_view: Float,
    pub transform: Matrix<4>,
    pub half_width: Float,
    pub half_height: Float,
    pub pixel_size : Float
}

impl Camera {
    pub fn new(h_size: usize, v_size: usize, field_of_view: Float) -> Camera {
        let half_view = (field_of_view / 2.0).tan() as Float;
        let aspect = h_size as Float / v_size as Float;

        let half_width: Float;
        let half_height: Float;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / h_size as Float;

        Camera{h_size, v_size, field_of_view, transform: Matrix::<4>::identity(), half_width, half_height, pixel_size }
    }

}