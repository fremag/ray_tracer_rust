use std::ops::Index;
use crate::intersection::Intersection;
use sorted_vec::SortedVec;

pub struct Intersections {
    pub intersections: SortedVec<Intersection>
}

pub fn intersections<'a>(intersections: Vec<Intersection>) -> Intersections {
    Intersections{ intersections: SortedVec::from_unsorted(intersections) }
}

impl Intersections {
    pub fn count(&self) -> usize {
        self.intersections.len()
    }

    pub fn hit(&self) -> Option<&Intersection> {
        for inter in self.intersections.iter() {
            if inter.t >= 0.0 {
                return Some(inter);
            }
        }
        None
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &(self.intersections[index])
    }
}
