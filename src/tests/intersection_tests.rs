#[cfg(test)]
use crate::canvas::Canvas;
use crate::colors::Color;
use crate::comps::prepare_computations;
use crate::intersection::Intersection;
use crate::intersections::{intersections};
use crate::math::{EPSILON, Float};
use crate::object::build_sphere;
use crate::ray::ray;
use crate::transform::translation;
use crate::tuple::{point, vector};

#[test]
fn an_intersection_encapsulates_t_and_object_test() {
    let s = build_sphere();
    let i = Intersection::new(3.5, &s);
    assert_eq!(i.t, 3.5);
    assert_eq!(i.object.shape(), s.shape());
}

#[test]
fn aggregating_intersections_test() {
    let s = build_sphere();
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);
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
    let i1 = Intersection { t: 1.0, object: &s };
    let i2 = Intersection { t: 2.0, object: &s };
    let xs = intersections(vec!(i2, i1));
    let i = xs.hit();
    match i {
        None => panic!(),
        Some(intersection) => assert_eq!(*intersection, i1)
    }
}

#[test]
fn the_hit_when_some_intersections_have_negative_t_test() {
    let s = build_sphere();
    let i1 = Intersection { t: -1.0, object: &s };
    let i2 = Intersection { t: 1.0, object: &s };
    let xs = intersections(vec!(i2, i1));
    let i = xs.hit();
    match i {
        None => panic!(),
        Some(intersection) => assert_eq!(*intersection, i2)
    }
}

#[test]
fn the_hit_when_all_intersections_have_negative_t_test() {
    let s = build_sphere();
    let i1 = Intersection { t: -2.0, object: &s };
    let i2 = Intersection { t: -1.0, object: &s };
    let xs = intersections(vec!(i2, i1));
    let i = xs.hit();
    assert_eq!(i, None);
}

#[test]
fn the_hit_is_always_the_lowest_non_negative_intersection_test() {
    let s = build_sphere();
    let i1 = Intersection { t: 5.0, object: &s };
    let i2 = Intersection { t: 7.0, object: &s };
    let i3 = Intersection { t: -3.0, object: &s };
    let i4 = Intersection { t: 2.0, object: &s };
    let xs = intersections(vec!(i1, i2, i3, i4));
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
        Ok(_) => { print!("Ok") }
        Err(error) => { print!("Error: {}", error) }
    }
}
#[test]
fn the_hit_should_offset_the_point_test() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut shape = build_sphere();
    shape.set_transformation(translation(0.0, 0.0, 1.0));
    let i = Intersection::new(5.0, &shape);
    let comps = prepare_computations(&i, &r);
    assert!(comps.over_point.z < -EPSILON / 2.0);
    assert!(comps.point.z > comps.over_point.z);
}

