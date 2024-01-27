use std::ops::Index;
use crate::core::intersection::Intersection;
use sorted_vec::SortedVec;

pub struct Intersections<'a> {
    pub intersections: SortedVec<Intersection<'a>>
}

pub fn intersections<'a>(intersections: Vec<Intersection>) -> Intersections {
    Intersections{ intersections: SortedVec::from_unsorted(intersections) }
}

impl<'a> Intersections<'a> {
    pub fn hit(&self) -> Option<&Intersection> {
        for inter in self.intersections.iter() {
            if inter.t >= 0.0 {
                return Some(inter);
            }
        }
        None
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &(self.intersections[index])
    }
}
