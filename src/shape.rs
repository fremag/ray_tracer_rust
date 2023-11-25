use std::fmt::Debug;

#[derive(PartialEq)]
pub enum ShapeType {Sphere}

pub trait Shape : Debug {

    fn shape_type(&self) -> ShapeType;
    fn equals(&self, other : &dyn Shape) -> bool;
}

impl PartialEq<Self> for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for dyn Shape {

}