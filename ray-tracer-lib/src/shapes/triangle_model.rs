use std::collections::HashSet;
use crate::core::bounds::Bounds;
use crate::core::math::Float;
use crate::core::matrix::Matrix;
use crate::core::ray::Ray;
use crate::object::Object;
use crate::shapes::triangle::Triangle;

#[derive(Debug, Clone)]
pub struct TriangleModel {
    pub triangles : Vec<Triangle>,
    pub bounds : Bounds,
    triangles_ids : HashSet<usize>
}

impl TriangleModel {
    pub fn new(triangles : Vec<Triangle>) -> Self {
        let mut model = Self {triangles, bounds: Bounds::new(), triangles_ids: HashSet::new()};
        model.init();
        model
    }

    pub(crate) fn set_transformation(& mut self, transformation: Matrix<4>) {
        let transformed_triangles = self.triangles.iter().map( |t| Triangle::new(
            &transformation * &t.p1,
            &transformation * &t.p2,
            &transformation * &t.p3
        )).collect();
        self.triangles = transformed_triangles;
        self.init();
    }

    pub(crate) fn bounds(&self) -> Bounds {
        self.bounds.clone()
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<(Float, Triangle)> {
        if self.bounds.intersect(&ray).is_empty() {
            return vec![];
        }
        let mut xs = vec![];

        for triangle in self.triangles.iter() {
            let child_xs = triangle.intersect(ray);
            for t in child_xs.iter() {
                let x = (*t, triangle.clone());
                xs.push(x);
            }
        }

        xs
    }

    fn init(&mut self) {
        let mut bounds = Bounds::new();

        for triangle in self.triangles.iter() {
            bounds.add(&triangle.p1);
            bounds.add(&triangle.p2);
            bounds.add(&triangle.p3);
            let id = triangle.id;
            self.triangles_ids.insert(id);
        }

        self.bounds = bounds;
    }

    pub(crate) fn includes(&self, object: &Object) -> bool {
        self.triangles_ids.contains(&object.object_id)
    }
}