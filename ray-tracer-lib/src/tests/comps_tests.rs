#[cfg(test)]
mod tests {
    use crate::core::comps::prepare_computations;
    use crate::core::intersection::Intersection;
    use crate::core::intersections::intersections;
    use crate::core::math::SQRT2;
    use crate::object::{build_plane, build_sphere};
    use crate::core::ray::ray;
    use crate::core::tuple::{point, vector};

    #[test]
    pub fn precomputing_the_state_of_an_intersection_test() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = build_sphere();
        let i = Intersection::new(4.0, shape);
        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    pub fn the_hit_when_an_intersection_occurs_on_the_outside_test() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = build_sphere();
        let i = Intersection::new(4.0, shape);

        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside_test() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = build_sphere();
        let i = Intersection::new(1.0, shape);
        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        // normal would have been(0, 0, 1), but is inverted!
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn precomputing_the_reflection_vector_test() {
        let shape = build_plane();
        let r = ray(point(0.0, 1.0, -1.0), vector(0.0, -SQRT2 / 2.0, SQRT2 / 2.0));
        let i = Intersection::new(SQRT2, shape);
        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        assert_eq!(comps.reflectv, vector(0.0, SQRT2 / 2.0, SQRT2 / 2.0));
    }
}