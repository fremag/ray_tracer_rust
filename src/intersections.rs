use std::ops::Index;
use crate::intersection::Intersection;

pub struct Intersections {
    pub intersections: Vec<Intersection>
}

pub fn intersections<'a>(intersections: Vec<Intersection>) -> Intersections {
    Intersections{intersections}
}

impl Intersections {
    pub fn count(&self) -> usize {
        self.intersections.len()
    }
}

impl<'a> Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &(self.intersections[index])
    }
}
