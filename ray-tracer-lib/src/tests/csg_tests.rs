#[cfg(test)]
mod tests {
    use crate::object::{build_cube, build_sphere};
    use crate::shapes::csg::{Csg, CsgOperation};

    #[test]
    fn csg_is_created_with_an_operation_and_two_shapes_test() {
        let s1 = build_sphere();
        let s2 = build_cube();
        let csg = Csg::new(CsgOperation::Union, s1.clone(), s2.clone());
        assert_eq!(csg.csg_operation, CsgOperation::Union);
        assert_eq!(*csg.left(), s1);
        assert_eq!(*csg.right(), s2);
    }

    #[test]
    fn evaluating_the_rule_for_a_csg_operation_union_test() {
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Union, true, true, true), false);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Union, true, true, false), true);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Union, true, false, true), false);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Union, true, false, false), true);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Union, false, true, true), false);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Union, false, true, false), false);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Union, false, false, true), true);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Union, false, false, false), true);
    }

    #[test]
    fn evaluating_the_rule_for_a_csg_operation_intersection_test() {
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Intersection, true, true, true), true);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Intersection, true, true, false), false);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Intersection, true, false, true), true);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Intersection, true, false, false), false);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Intersection, false, true, true), true);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Intersection, false, true, false), true);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Intersection, false, false, true), false);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Intersection, false, false, false), false);
    }

    #[test]
    fn evaluating_the_rule_for_a_csg_operation_difference_test() {
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Difference, true, true, true), false);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Difference, true, true, false), true);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Difference, true, false, true), false);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Difference, true, false, false), true);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Difference, false, true, true), true);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Difference, false, true, false), true);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Difference, false, false, true), false);
        assert_eq!(Csg::intersection_allowed(&CsgOperation::Difference, false, false, false), false);
    }
}