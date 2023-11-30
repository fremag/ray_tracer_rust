use std::ops::Index;
use crate::intersection::Intersection;

pub struct Intersections<'a> {
    pub intersections: Vec<Intersection<'a>>
}

pub fn intersections<'a>(intersections: Vec<Intersection<'a>>) -> Intersections<'a> {
    Intersections{intersections}
}

impl<'a> Intersections<'a> {
    pub fn count(&self) -> usize {
        self.intersections.len()
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &(self.intersections[index])
    }
}
