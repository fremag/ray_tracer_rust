#[cfg(test)]
mod tests {
    use crate::material::Material;
    use crate::core::math::{PI, SQRT2};
    use crate::core::matrix::Matrix;
    use crate::object::{build_sphere, get_next_unique_shape_id, Object};
    use crate::core::ray::ray;
    use crate::shapes::shape::Shape;
    use crate::shapes::sphere;
    use crate::core::transform::{rotation_z, scaling, translation};
    use crate::core::tuple::{point, vector};
    use crate::shapes::sphere::sphere;
    use crate::tests::helper::tests::dummy_intersection;

    #[test]
    fn a_sphere_s_default_transformation_test() {
        let obj_s = build_sphere();
        assert_eq!(obj_s.transformation(), &Matrix::<4>::identity());
        assert_eq!(obj_s.transformation_inverse(), &Matrix::<4>::identity());
        assert_eq!(obj_s.shape(), Some(&sphere()));
    }

    #[test]
    fn changing_a_sphere_transformation_test() {
        let mut obj_s = build_sphere();
        obj_s.set_transformation(translation(2.0, 3.0, 4.0));

        assert_eq!(obj_s.transformation(), &translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray_test() {
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
        let s = build_sphere();
        let m = s.material();
        assert_eq!(m, &Material::new());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material_test() {
        let mut s = build_sphere();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.set_material(m);
        assert_eq!(s.material(), &m);
    }

    #[test]
    fn the_default_transformation_test() {
        let sphere = sphere::Sphere {};
        let s = Object::new(Shape::Sphere(sphere));
        assert_eq!(s.transformation(), &Matrix::<4>::identity());
    }

    #[test]
    fn assigning_a_transformation_test() {
        let sphere = sphere::Sphere {};
        let mut s = Object::new(Shape::Sphere(sphere));
        s.set_transformation(translation(2.0, 3.0, 4.0));
        assert_eq!(s.transformation(), &translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn the_default_material_test() {
        let sphere = sphere::Sphere {};
        let s = Object::new(Shape::Sphere(sphere));
        let m = s.material();
        assert_eq!(m, &Material::new());
    }

    #[test]
    fn assigning_a_material_test() {
        let sphere = sphere::Sphere {};
        let mut s = Object::new(Shape::Sphere(sphere));
        let mut m = Material::new();
        m.ambient = 1.0;
        s.set_material(m);
        assert_eq!(s.material(), &m)
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray_test() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = sphere::Sphere {};
        let mut s = Object::new(Shape::Sphere(sphere));
        s.set_transformation(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        match xs.hit() {
            None => {}
            Some(_) => {}
        }
        // assert_eq!( s.saved_ray.origin, point(0.0, 0.0, -2.5));
        // assert_eq!( s.saved_ray.direction = vector(0.0, 0.0, 0.5)
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray_test() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = sphere::Sphere {};
        let mut s = Object::new(Shape::Sphere(sphere));
        s.set_transformation(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        match xs.hit() {
            None => {}
            Some(_) => {}
        }
        // assert_eq!( s.saved_ray.origin, point(-5.0, 0.0, -5.0));
        // assert_eq!( s.saved_ray.direction, vector(0.0, 0.0, 1.0));
    }


    #[test]
    fn computing_the_normal_on_a_translated_shape_test() {
        let sphere = sphere::Sphere {};
        let mut s = Object::new(Shape::Sphere(sphere));
        s.set_transformation(translation(0.0, 1.0, 0.0));

        let n = s.normal_at(point(0.0, 1.0 + SQRT2 / 2.0, -SQRT2 / 2.0), &dummy_intersection() );
        assert_eq!(n, vector(0.0, SQRT2 / 2.0, -SQRT2 / 2.0));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape_test() {
        let sphere = sphere::Sphere {};
        let mut s = Object::new(Shape::Sphere(sphere));
        let m = &scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0);
        s.set_transformation(m);
        let n = s.normal_at(point(0.0, SQRT2 / 2.0, -SQRT2 / 2.0), &dummy_intersection());
        assert_eq!(n, vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn object_counter_test() {
        let id1 = get_next_unique_shape_id();
        let id2 = get_next_unique_shape_id();
        assert_eq!(id1+1, id2);
    }
}