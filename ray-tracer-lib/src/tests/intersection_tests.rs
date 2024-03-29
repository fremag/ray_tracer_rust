#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::colors::Color;
    use crate::core::comps::prepare_computations;
    use crate::core::intersection::Intersection;
    use crate::core::intersections::{intersections, Intersections};
    use crate::core::math::{EPSILON, equals, Float, SQRT2};
    use crate::core::ray::{ray, Ray};
    use crate::core::transform::{scaling, translation};
    use crate::core::tuple::{point, vector};
    use crate::material::Material;
    use crate::object::{build_sphere, Object};
    use crate::shapes::triangle::Triangle;
    use crate::shapes::triangle_model::TriangleModel;
    use crate::tests::helper::tests::build_glass_sphere;

    #[test]
    fn an_intersection_encapsulates_t_and_object_test() {
        let s = build_sphere();
        let i = Intersection::new(3.5, s.clone());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object.shape(), s.shape());
    }

    #[test]
    fn aggregating_intersections_test() {
        let s = build_sphere();
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());
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
        let s = build_sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].object.shape(), s.shape());
        assert_eq!(xs[1].object.shape(), s.shape());
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t_test() {
        let s = build_sphere();
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());
        let xs = intersections(vec!(i2.clone(), i1.clone()));
        let i = xs.hit();
        match i {
            None => panic!(),
            Some(intersection) => assert_eq!(*intersection, i1.clone())
        }
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t_test() {
        let s = build_sphere();
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s.clone());
        let xs = intersections(vec!(i2.clone(), i1.clone()));
        let i = xs.hit();
        match i {
            None => panic!(),
            Some(intersection) => assert_eq!(*intersection, i2)
        }
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t_test() {
        let s = build_sphere();
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s);
        let xs = intersections(vec!(i2, i1));
        let i = xs.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection_test() {
        let s = build_sphere();
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s.clone());
        let xs = intersections(vec!(i1, i2, i3, i4.clone()));
        let i = xs.hit();
        match i {
            None => panic!(),
            Some(intersection) => assert_eq!(*intersection, i4)
        }
    }

    #[test]
    fn putting_it_together_test() {

        // start the ray at z = -5
        let ray_origin = point(0.0, 0.0, -5.0);
        //  put the wall at z = 10
        let wall_z = 10.0;
        let wall_size = 7.0;
        let canvas_pixels = 100;
        let pixel_size = wall_size / canvas_pixels as Float;
        let half = wall_size / 2.0;
        let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
        let color = Color::new(1.0, 0.0, 0.0); // red
        let sphere = build_sphere();

        // for each row of pixels in the canvas
        for y in 0..canvas_pixels - 1 {
            // compute the world y coordinate (top = +half, bottom = -half)
            let world_y = half - pixel_size * y as Float;
            // for each pixel in the row
            for x in 0..canvas_pixels - 1 {
                // compute the world x coordinate(left = -half, right = half)
                let world_x = -half + pixel_size * x as Float;
                // describe the point on the wall that the ray will target
                let position = point(world_x, world_y, wall_z);
                let r = ray(ray_origin, (position - ray_origin).normalize());
                let xs = sphere.intersect(&r);
                match xs.hit() {
                    None => { /* no intersection, do nothing */ }
                    Some(_) => { canvas.write_pixel(x, y, color) }
                }
            }
        }

        let result = canvas.save("e:\\tmp\\sphere_silhouette.ppm");
        match result {
            Ok(_) => { println!("Ok") }
            Err(error) => { println!("Error: {}", error) }
        }
    }

    #[test]
    fn the_hit_should_offset_the_point_test() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = build_sphere();
        shape.set_transformation(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, shape.clone());
        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }


    #[test]
    fn finding_n1_and_n2_at_various_intersections_test() {
        let mut a = build_glass_sphere();
        let mut b = build_glass_sphere();
        let mut c = build_glass_sphere();
        a.set_transformation(scaling(2.0, 2.0, 2.0));
        let mut mat_a = Material::new();
        mat_a.refractive_index = 1.5;
        a.set_material(mat_a);

        b.set_transformation(translation(0.0, 0.0, -0.25));
        let mut mat_b = Material::new();
        mat_b.refractive_index = 2.0;
        b.set_material(mat_b);
        c.set_transformation(translation(0.0, 0.0, 0.25));
        let mut mat_c = Material::new();
        mat_c.refractive_index = 2.5;
        c.set_material(mat_c);

        let r = ray(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
        let xs = intersections(vec!(
            Intersection::new(2.0, a.clone()),
            Intersection::new(2.75, b.clone()),
            Intersection::new(3.25, c.clone()),
            Intersection::new(4.75, b.clone()),
            Intersection::new(5.25, c.clone()),
            Intersection::new(6.00, a.clone())));

        check_n1_n2(&xs, &r, 0, 1.0, 1.5);
        check_n1_n2(&xs, &r, 1, 1.5, 2.0);
        check_n1_n2(&xs, &r, 2, 2.0, 2.5);
        check_n1_n2(&xs, &r, 3, 2.5, 2.5);
        check_n1_n2(&xs, &r, 4, 2.5, 1.5);
        check_n1_n2(&xs, &r, 5, 1.5, 1.0);
    }

    fn check_n1_n2(xs: &Intersections, r: &Ray, index: usize, n1: Float, n2: Float) {
        let comps = prepare_computations(&xs.intersections[index], &r, &xs);
        assert_eq!(comps.n1, n1);
        assert_eq!(comps.n2, n2);
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface_test() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = build_glass_sphere();
        shape.set_transformation(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, shape.clone());
        let xs = intersections(vec![i.clone()]);
        let comps = prepare_computations(&i, &r, &xs);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection_test() {
        let shape = build_glass_sphere();
        let r = ray(point(0.0, 0.0, SQRT2 / 2.0), vector(0.0, 1.0, 0.0));
        let xs = intersections(vec!(Intersection::new(-SQRT2 / 2.0, shape.clone()), Intersection::new(SQRT2 / 2.0, shape.clone())));
        let comps = prepare_computations(&xs.intersections[1], &r, &xs);
        let reflectance = comps.schlick();
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle_test() {
        let shape = build_glass_sphere();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = intersections(vec![Intersection::new(-1.0, shape.clone()), Intersection::new(1.0, shape.clone())]);
        let comps = prepare_computations(&xs[1], &r, &xs);
        let reflectance = comps.schlick();
        assert!(equals(reflectance, 0.04));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = build_glass_sphere();
        let r = ray(point(0.0, 0.99, -2.0), vector(0.0, 0.0, 1.0));
        let xs = intersections(vec![Intersection::new(1.8589, shape.clone())]);
        let comps = prepare_computations(&xs.intersections[0], &r, &xs);
        let reflectance = comps.schlick();
        assert!(equals(reflectance, 0.48873));
    }


    #[test]
    fn an_intersection_can_encapsulate_u_and_v_test() {
        let s = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
        let model = TriangleModel::new(vec![s]);
        let i = Intersection::new_uv(3.5, Object::new_triangle_group(model), 0.2, 0.4);
        assert_eq!(i.u, 0.2);
        assert_eq!(i.v, 0.4);
    }
}