#[cfg(test)]

use crate::bounds::Bounds;
use crate::transform::{scaling, translation};
use crate::tuple::point;

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