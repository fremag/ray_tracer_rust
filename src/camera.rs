use crate::canvas::Canvas;
use crate::math::Float;
use crate::matrix::Matrix;
use crate::ray::{Ray, ray};
use crate::tuple::point;
use crate::world::World;

pub struct Camera {
    pub h_size: usize,
    pub v_size: usize,
    pub field_of_view: Float,
    pub half_width: Float,
    pub half_height: Float,
    pub pixel_size: Float,
    pub transform: Matrix<4>,
    pub inverse_transform: Matrix<4>,
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

        Camera { h_size, v_size, field_of_view, transform: Matrix::<4>::identity(), inverse_transform: Matrix::<4>::identity(), half_width, half_height, pixel_size }
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
        self.inverse_transform = self.transform.inverse();
    }

    pub(crate) fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let xoffset = (px as Float + 0.5) * self.pixel_size;
        let yoffset = (py as Float + 0.5) * self.pixel_size;
        // the untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1)
        let pixel = &self.inverse_transform * &point(world_x, world_y, -1.0);
        let origin = &self.inverse_transform * &point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        ray(origin, direction)
    }

    pub(crate) fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.h_size, self.v_size);
        for y in 0..self.v_size {
            for x in 0..self.h_size {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, 1);
                image.write_pixel(x, y, color);
            }
        }

        image
    }
}