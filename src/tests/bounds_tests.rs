#[cfg(test)]

use crate::bounds::Bounds;
use crate::math::Float;
use crate::ray::ray;
use crate::transform::{scaling, translation};
use crate::tuple::{point, vector};

#[test]
fn transform_scaling_test() {
    let bounds = Bounds::from(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0));
    let transformation = scaling(2.0, 3.0, 4.0,);
    let transformed = bounds.transform(&transformation);
    assert_eq!(transformed.min, point(-2.0, -3.0, -4.0));
    assert_eq!(transformed.max, point(2.0, 3.0, 4.0));
}

#[test]
fn transform_translating_test() {
    let bounds = Bounds::from(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0));
    let transformation = translation(2.0, 3.0, 4.0,);
    let transformed = bounds.transform(&transformation);
    assert_eq!(transformed.min, point(1.0, 2.0, 3.0));
    assert_eq!(transformed.max, point(3.0, 4.0, 5.0));
}

#[test]
fn extend_smaller_test() {
    let mut bounds = Bounds::from(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0));
    let small_bounds = Bounds::from(point(-0.5, -0.5, -0.5), point(0.5, 0.5, 0.5));
    bounds.extend(&small_bounds);
    assert_eq!(bounds.min, point(-1.0, -1.0, -1.0));
    assert_eq!(bounds.max, point(1.0, 1.0, 1.0));
}

#[test]
fn extend_larger_x_test() {
    let mut bounds = Bounds::from(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0));
    let larger_bounds = Bounds::from(point(-5.0, -0.5, -0.5), point(4.0, 0.5, 0.5));
    bounds.extend(&larger_bounds);
    assert_eq!(bounds.min, point(-5.0, -1.0, -1.0));
    assert_eq!(bounds.max, point(4.0, 1.0, 1.0));
}

#[test]
fn add_test() {
    let mut bounds = Bounds::from(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0));
    bounds.add(&point(-3.0, 0.0, 0.0));
    assert_eq!(bounds.min, point(-3.0, -1.0, -1.0));
    assert_eq!(bounds.max, point(1.0, 1.0, 1.0));

    bounds.add(&point(0.0, 5.0, 0.0));
    assert_eq!(bounds.min, point(-3.0, -1.0, -1.0));
    assert_eq!(bounds.max, point(1.0, 5.0, 1.0));
}

#[test]
fn intersect_test() {
    check_intersection(-10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 9.0, 14.0);
    check_intersection(0.0, -10.0, 0.0, 0.0, 1.0, 0.0, 8.0, 15.0);
    check_intersection(0.0, 0.0, 10.0, 0.0, 0.0, -1.0, 4.0, 13.0);
}

fn check_intersection(px: Float, py: Float, pz: Float, dx: Float, dy: Float, dz: Float, t1: Float, t2: Float) {
    let bounds = Bounds::from(point(-1.0, -2.0, -3.0), point(4.0, 5.0, 6.0));
    let r = ray(point(px, py, pz), vector(dx, dy, dz));
    let xs = bounds.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], t1);
    assert_eq!(xs[1], t2);
}
