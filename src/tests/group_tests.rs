#[cfg(test)]
use crate::group::Group;
use crate::matrix::Matrix;
use crate::object::{build_glass_sphere, build_sphere, Object};
use crate::ray::ray;
use crate::transform::{scaling, translation};
use crate::tuple::{point, vector};

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

#[test]
fn intersecting_a_ray_with_an_empty_group_test() {
    let g = Group::new();
    let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let xs = g.intersect(&r);
    assert!(xs.intersections.is_empty());
}

#[test]
fn intersecting_a_ray_with_a_nonempty_group_test() {
    let mut g = Group::new();
    let s1 = build_sphere();
    let mut s2 = build_sphere();
    s2.set_transformation(translation(0.0, 0.0, -3.0));
    let mut s3 = build_sphere();
    s3.set_transformation(translation(5.0, 0.0, 0.0));
    g.add(&s1);
    g.add(&s2);
    g.add(&s3);

    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let xs = g.intersect(&r);
    assert_eq!(xs.intersections.len(), 4);
    assert_eq!(xs.intersections[0].t, 1.0);
    assert_eq!(xs.intersections[1].t, 3.0);
    assert_eq!(xs.intersections[2].t, 4.0);
    assert_eq!(xs.intersections[3].t, 6.0);

    assert_eq!(xs.intersections[0].object, &s2);
    assert_eq!(xs.intersections[1].object, &s2);
    assert_eq!(xs.intersections[2].object, &s1);
    assert_eq!(xs.intersections[3].object, &s1);
}


#[test]
fn intersecting_a_transformed_group_test() {
    let mut g = Group::new();
    g.set_transformation(scaling(2.0, 2.0, 2.0));
    let mut s = build_sphere();
    s.set_transformation(translation(5.0, 0.0, 0.0));
    g.add(&s);
    let r = ray(point(10.0, 0.0, -10.0), vector(0.0, 0.0, 1.0));
    let xs = g.intersect(&r);
    assert_eq!(xs.intersections.len(), 2);
}