#[cfg(test)]
use crate::material::material;
use crate::matrix::Matrix;
use crate::object::build_sphere;
use crate::ray::ray;
use crate::sphere::sphere;
use crate::transform::{scaling, translation};
use crate::tuple::{point, vector};

#[test]
fn a_sphere_s_default_transformation_test() {
    let obj_s = build_sphere();
    assert_eq!(obj_s.transformation(), &Matrix::<4>::identity());
    assert_eq!(obj_s.transformation_inverse(), &Matrix::<4>::identity());
    assert_eq!(obj_s.shape(), sphere());
}

#[test]
fn changing_a_sphere_transformation_test() {
    let mut obj_s = build_sphere();
    obj_s.set_transformation(translation(2.0, 3.0, 4.0));

    assert_eq!(obj_s.transformation(), &translation(2.0, 3.0, 4.0));
}

#[test]
fn  intersecting_a_scaled_sphere_with_a_ray_test() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut obj_s = build_sphere();
    obj_s.set_transformation(scaling(2.0, 2.0, 2.0));

    let xs = obj_s.intersect(&r);
    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
fn intersecting_a_translated_sphere_with_a_ray_test() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = build_sphere();
    s.set_transformation(translation(5.0, 0.0, 0.0));

    let xs = s.intersect(&r);
    assert_eq!(xs.count(), 0);
}

#[test]
fn a_sphere_has_a_default_material_test() {
    let s =build_sphere();
    let m= s.material();
    assert_eq!(m, &material());
}
#[test]
fn a_sphere_may_be_assigned_a_material_test() {
    let mut s = build_sphere();
    let mut m= material();
    m.ambient = 1.0;
    s.set_material(m);
    assert_eq!(s.material(), &m);
}
