use crate::tuple::{Tuple, vector, point};

#[test]
pub fn is_point() {
    let point = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: 1.0
    };

    assert_eq!(point.x, 4.3);
    assert_eq!(point.y, -4.2);
    assert_eq!(point.z, 3.1);
    assert_eq!(point.w, 1.0);
    assert_eq!(point.is_point(), true);
    assert_eq!(point.is_vector(), false);
}

#[test]
pub fn is_vector() {
    let vector = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: 0.0
    };

    assert_eq!(vector.x, 4.3);
    assert_eq!(vector.y, -4.2);
    assert_eq!(vector.z, 3.1);
    assert_eq!(vector.w, 0.0);
    assert_eq!(vector.is_point(), false);
    assert_eq!(vector.is_vector(), true);
}

#[test]
pub fn vector_test() {
    let v = vector(4.0, -4.0, 3.0);
    assert_eq!(v.x, 4.0);
    assert_eq!(v.y, -4.0);
    assert_eq!(v.z, 3.0);
    assert_eq!(v.is_vector(), true);
}

#[test]
pub fn point_test() {
    let p = point(4.0, -4.0, 3.0);
    assert_eq!(p.x, 4.0);
    assert_eq!(p.y, -4.0);
    assert_eq!(p.z, 3.0);
    assert_eq!(p.is_point(), true);
}

#[test]
pub fn equals_test() {
    let p1 = point(4.0, -4.0, 3.0);
    let p2 = point(4.0, -4.0, 3.0);
    let p3 = point(4.001, -4.0, 3.0);
    let v1 = vector(4.0, -4.0, 3.0);
    assert_eq!(p1, p2);
    assert_eq!(p1 == p2, true);
    assert_eq!(p1 == p3, false);
    assert_eq!(p1 == v1, false);
}