use crate::core::bounds::Bounds;
use crate::core::math::Float;
use crate::core::matrix::Matrix;
use crate::core::ray::Ray;
use crate::shapes::smooth_triangle::SmoothTriangle;

#[derive(Debug, Clone)]
pub struct SmoothTriangleModel {
    pub smooth_triangles : Vec<SmoothTriangle>,
    pub bounds : Bounds,
}

impl SmoothTriangleModel {
    pub fn new(smooth_triangles : Vec<SmoothTriangle>) -> Self {
        let mut model = Self {smooth_triangles, bounds: Bounds::new()};
        model.init();
        model
    }

    pub(crate) fn set_transformation(& mut self, transformation: Matrix<4>) {
        let transformed_triangles = self.smooth_triangles.iter().map( |t| SmoothTriangle::new(
            &transformation * &t.triangle.p1,
            &transformation * &t.triangle.p2,
            &transformation * &t.triangle.p3,
            &transformation * &t.n1,
            &transformation * &t.n2,
            &transformation * &t.n3,
        )).collect();
        self.smooth_triangles = transformed_triangles;
        self.init();
    }

    pub(crate) fn bounds(&self) -> Bounds {
        self.bounds.clone()
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<(Float, SmoothTriangle, Float, Float)> {
        if self.bounds.intersect(&ray).is_empty() {
            return vec![];
        }
        let mut xs = vec![];

        for smooth_triangle in self.smooth_triangles.iter() {
            let (t, u, v) = smooth_triangle.intersect_uv(ray);
            if ! Float::is_nan(t) {
                xs.push((t, smooth_triangle.clone(), u, v));
            }
        }

        xs
    }

    fn init(&mut self) {
        let mut bounds = Bounds::new();

        for triangle in self.smooth_triangles.iter() {
            bounds.add(&triangle.triangle.p1);
            bounds.add(&triangle.triangle.p2);
            bounds.add(&triangle.triangle.p3);
        }

        self.bounds = bounds;
    }
}