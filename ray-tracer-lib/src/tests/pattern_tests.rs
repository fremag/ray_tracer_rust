#[cfg(test)]
mod tests {
    use crate::colors::Color;
    use crate::core::matrix::Matrix;
    use crate::object::build_sphere;
    use crate::patterns::checker::CheckerPattern;
    use crate::patterns::gradient::GradientPattern;
    use crate::patterns::pattern::{Pattern};
    use crate::patterns::ring::RingPattern;
    use crate::patterns::stripe::StripePattern;
    use crate::core::transform::{scaling, translation};
    use crate::core::tuple::point;

    #[test]
    fn creating_a_stripe_pattern_test() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.a, Color::white());
        assert_eq!(pattern.b, Color::black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y_test() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z_test() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x_test() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(&point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(&point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(&point(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn pattern_with_an_object_transformation_test() {
        let mut object = build_sphere();
        object.set_transformation(scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::stripe(Color::white(), Color::black());
        let c = pattern.pattern_at_object(&object, point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }

    #[test]
    fn pattern_with_a_pattern_transformation_test() {
        let object = build_sphere();
        let mut pattern = Pattern::stripe(Color::white(), Color::black());
        pattern.set_pattern_transform(&scaling(2.0, 2.0, 2.0));
        let c = pattern.pattern_at_object(&object, point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }

    #[test]
    fn pattern_with_both_an_object_and_a_pattern_transformation_test() {
        let mut object = build_sphere();
        object.set_transformation(scaling(2.0, 2.0, 2.0));
        let mut pattern = Pattern::stripe(Color::white(), Color::black());
        pattern.set_pattern_transform(&translation(0.5, 0.0, 0.0));
        let c = pattern.pattern_at_object(&object, point(2.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }

    #[test]
    fn the_default_pattern_transformation_test() {
        let pattern = Pattern::new();
        assert_eq!(pattern.inverse_transform, Matrix::<4>::identity());
    }

    #[test]
    fn assigning_a_transformation_test() {
        let mut pattern = Pattern::new();
        pattern.set_pattern_transform(&translation(1.0, 2.0, 3.0));
        assert_eq!(pattern.inverse_transform, translation(1.0, 2.0, 3.0).inverse());
    }

    #[test]
    fn pattern_with_an_object_transformation_2_test() {
        let mut object = build_sphere();
        object.set_transformation(scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::test();
        let c = pattern.pattern_at_object(&object, point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_a_pattern_transformation_2_test() {
        let object = build_sphere();
        let mut pattern = Pattern::test();
        pattern.set_pattern_transform(&scaling(2.0, 2.0, 2.0));
        let c = pattern.pattern_at_object(&object, point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_both_an_object_and_a_pattern_transformation_2_test() {
        let mut object = build_sphere();
        object.set_transformation(scaling(2.0, 2.0, 2.0));
        let mut pattern = Pattern::test();
        pattern.set_pattern_transform(&translation(0.5, 1.0, 1.50));
        let c = pattern.pattern_at_object(&object, point(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors_test() {
        let pattern = GradientPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.25, 0.0, 0.0)), Color::new(0.75, 0.75, 0.75));
        assert_eq!(pattern.pattern_at(&point(0.5, 0.0, 0.0)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(pattern.pattern_at(&point(0.75, 0.0, 0.0)), Color::new(0.25, 0.25, 0.25));
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z_test() {
        let pattern = RingPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 1.0)), Color::black());
        // 0.708 = just slightly more than √ 2/2
        assert_eq!(pattern.pattern_at(&point(0.708, 0.0, 0.708)), Color::black());
    }

    #[test]
    fn checkers_should_repeat_in_x_test() {
        let pattern = CheckerPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.99, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(1.01, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn checkers_should_repeat_in_y_test() {
        let pattern = CheckerPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.99, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 1.01, 0.0)), Color::black());
    }

    #[test]
    fn checkers_should_repeat_in_z_test() {
        let pattern = CheckerPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.99)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 1.01)), Color::black());
    }
}