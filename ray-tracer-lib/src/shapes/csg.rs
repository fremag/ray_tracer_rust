use crate::core::bounds::Bounds;
use crate::core::intersection::Intersection;
use crate::core::intersections::{Intersections, intersections};
use crate::core::matrix::Matrix;
use crate::core::ray::Ray;
use crate::object::Object;
use crate::shapes::group::Group;

#[derive(Debug, PartialEq, Clone)]
pub enum CsgOperation { Union, Intersection, Difference }

#[derive(Debug, Clone)]
pub struct Csg {
    pub csg_operation: CsgOperation,
    pub group: Group,
}

impl Csg {
    pub(crate) fn intersect(&self, ray: &Ray) -> Intersections {
        let intersections_with_bounds = self.group.bounds().intersect(&ray);
        if intersections_with_bounds.is_empty() {
            return intersections(vec![]);
        }
        
        let left = self.left();
        let mut left_xs = left.intersect(ray);
        let right = self.right();
        let right_xs = right.intersect(ray);

        for intersection in right_xs.intersections.iter() {
            left_xs.push(intersection.clone());
        }

        self.filter_intersections(left_xs)
    }

    pub(crate) fn includes(&self, object: &Object) -> bool {
        self.group.includes(object)
    }

    pub(crate) fn bounds(&self) -> Bounds {
        self.group.bounds()
    }

    pub(crate) fn set_transformation(&mut self, transformation: Matrix<4>) {
        self.group.set_transformation(transformation);
    }

    pub fn new(csg_operation: CsgOperation, left: Object, right: Object) -> Self {
        let mut group = Group::new();
        group.add(left);
        group.add(right);
        Csg { csg_operation, group }
    }

    pub fn left(&self) -> &Object {
        self.group.child(0)
    }

    pub fn right(&self) -> &Object {
        self.group.child(1)
    }

    pub fn intersection_allowed(op: &CsgOperation, lhit: bool, inl: bool, inr: bool) -> bool {
        match op {
            CsgOperation::Union => (lhit && !inr) || (!lhit && !inl),
            CsgOperation::Intersection => (lhit && inr) || (!lhit && inl),
            CsgOperation::Difference => (lhit && !inr) || (!lhit && inl),
        }
    }

    pub fn filter_intersections(&self, xs : Intersections) -> Intersections {
        // begin outside both children
        let mut inl = false;
        let mut inr = false;
        // prepare a list to receive the filtered intersections
        let mut result = intersections(vec![]);
        for i in xs.intersections.iter() {
            // if i.object is part of the "left" child, then lhit is true
            let obj = &i.object;
            let left = self.left();
            let lhit = left.includes(obj);
            if Csg::intersection_allowed(&self.csg_operation, lhit, inl, inr) {
                let intersection = Intersection::new(i.t, i.object.clone());
                result.intersections.push(intersection);
            }
            // depending on which object was hit, toggle either inl or inr
            if lhit {
                inl = ! inl;
            } else {
                inr = ! inr;
            }
        }

        result
    }

    pub(crate) fn get_child_ids(&self) -> Vec<usize> {
        let mut ids : Vec<usize> = self.left().get_child_ids().into_iter().collect();
        let ids_right = self.right().get_child_ids();
        ids.extend(ids_right);

        ids
    }
}