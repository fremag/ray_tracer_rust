use crate::group::Group;
use crate::object::{build_glass_sphere, Object};
use crate::shape::Shape;

#[cfg(test)]

#[test]
fn group_test() {
    let group = Group::new();
    let group_obj = Object::new(Shape::Group(group));
    let mut sphere = build_glass_sphere();

    group_obj.add(&mut sphere);
}
