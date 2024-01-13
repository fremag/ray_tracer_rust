#[cfg(test)]
use crate::group::Group;
use crate::matrix::Matrix;
use crate::object::{build_glass_sphere, Object};


#[test]
fn creating_a_new_group_test() {
    let group = Group::new();
    let group_object = Object::new_group(group);
    assert_eq!(group_object.transformation(), &Matrix::<4>::identity());
    assert_eq!(group_object.group().unwrap().len(), 0)
}

#[test]
fn adding_a_child_to_a_group_test() {
    let mut group = Group::new();
    let mut sphere1 = build_glass_sphere();
    let mut sphere2 = build_glass_sphere();
    let mut sphere3 = build_glass_sphere();

    group.add(&mut sphere1);
    group.add(&mut sphere2);
    group.add(&mut sphere3);

    assert_eq!(group.len(), 3)
}
