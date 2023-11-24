use std::fmt::Debug;

pub trait Shape : Debug {

}

impl PartialEq<Self> for dyn Shape {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for dyn Shape {

}