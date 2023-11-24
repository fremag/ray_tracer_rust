#[cfg(test)]
mod intersection_tests {
    use crate::intersection::Intersection;
    use crate::sphere::sphere;

    #[test]
    fn an_intersection_encapsulates_t_and_object_test() {
        let s = sphere();
        let i = Intersection::new(3.5,&s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.shape, &s);
    }
}

