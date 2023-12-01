#[cfg(test)]
mod intersection_tests {
    use crate::intersection::Intersection;
    use crate::intersections::{intersections};
    use crate::ray::ray;
    use crate::sphere::sphere;
    use crate::tuple::{point, vector};

    #[test]
    fn an_intersection_encapsulates_t_and_object_test() {
        let s = sphere();
        let i = Intersection::new(3.5, s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.shape, s);
    }

    #[test]
    fn aggregating_intersections_test() {
        let s = sphere();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = intersections(vec![i1, i2]);
        assert_eq!(xs.count(), 2);
        let intersection0 = &(xs[0]);
        let intersection1 = &(xs[1]);
        assert_eq!(intersection0.t, 1.0);
        assert_eq!(intersection1.t, 2.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection_test() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].shape, s);
        assert_eq!(xs[1].shape, s);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t_test() {
        let s = sphere();
        let i1 = Intersection { t: 1.0, shape: s };
        let i2 = Intersection { t: 2.0, shape: s };
        let xs = intersections(vec!(i2, i1));
        let i = xs.hit();
        match i {
            None => panic!(),
            Some(intersection) => assert_eq!(*intersection, i1)
        }
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t_test() {
        let s = sphere();
        let i1 = Intersection { t: -1.0, shape: s };
        let i2 = Intersection { t: 1.0, shape: s };
        let xs = intersections(vec!(i2, i1));
        let i = xs.hit();
        match i {
            None => panic!(),
            Some(intersection) => assert_eq!(*intersection, i2)
        }
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t_test() {
        let s = sphere();
        let i1 = Intersection { t: -2.0, shape: s };
        let i2 = Intersection { t: -1.0, shape: s };
        let xs = intersections(vec!(i2, i1));
        let i = xs.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection_test() {
        let s = sphere();
        let i1 = Intersection { t: 5.0, shape: s };
        let i2 = Intersection { t: 7.0, shape: s };
        let i3 = Intersection { t: -3.0, shape: s };
        let i4 = Intersection { t: 2.0, shape: s };
        let xs = intersections(vec!(i1, i2, i3, i4));
        let i = xs.hit();
        match i {
            None => panic!(),
            Some(intersection) => assert_eq!(*intersection, i4)
        }
    }
}

