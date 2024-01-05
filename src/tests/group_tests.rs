use crate::group::Group;
use crate::object::{build_glass_sphere, Object};
use crate::shape::Shape;

#[cfg(test)]

#[test]
fn group_test() {
    let group = Group::new();
    let mut group_obj = Object::new(Shape::Group(group));
    let mut sphere1 = build_glass_sphere();
    let mut sphere2 = build_glass_sphere();
    let mut sphere3 = build_glass_sphere();

    group_obj.add(&mut sphere1);
    group_obj.add(&mut sphere2);
    group_obj.add(&mut sphere3);
}
