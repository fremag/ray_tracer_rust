#[cfg(test)]

use crate::bounds::Bounds;
use crate::transform::scaling;
use crate::tuple::point;

#[test]
fn transform_test() {
    let bounds = Bounds::from(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0));
    let transformation = scaling(2.0, 3.0, 4.0,);
    let transformed = bounds.transform(&transformation);
    assert_eq!(transformed.max, point(2.0, 3.0, 4.0));
}