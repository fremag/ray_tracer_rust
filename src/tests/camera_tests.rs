#[cfg(test)]
use crate::camera::Camera;
use crate::colors::Color;
use crate::light::PointLight;
use crate::material::material;
use crate::math::{PI, SQRT2};
use crate::matrix::Matrix;
use crate::object::build_sphere;
use crate::transform::{rotation_x, rotation_y, scaling, translation, view_transform};
use crate::tuple::{point, vector};
use crate::world::{build_world, World};

#[test]
fn constructing_a_camera_test() {
    let h_size = 160;
    let v_size = 120;
    let field_of_view = PI / 2.0;
    let c = Camera::new(h_size, v_size, field_of_view);
    assert_eq!(c.h_size, 160);
    assert_eq!(c.v_size, 120);
    assert_eq!(c.field_of_view, PI / 2.0);
    assert_eq!(c.transform, Matrix::<4>::identity());
}

#[test]
fn the_pixel_size_for_a_horizontal_canvas_test() {
    let c = Camera::new(200, 125, PI / 2.0);
    assert_eq!(c.pixel_size, 0.01);
}

#[test]
fn the_pixel_size_for_a_vertical_canvas_test() {
    let c = Camera::new(125, 200, PI / 2.0);
    assert_eq!(c.pixel_size, 0.01);
}

#[test]
fn constructing_a_ray_through_the_center_of_the_canvas_test() {
    let c = Camera::new(201, 101, PI / 2.0);

    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, point(0.0, 0.0, 0.0));
    assert_eq!(r.direction, vector(0.0, 0.0, -1.0));
}

#[test]
fn constructing_a_ray_through_a_corner_of_the_canvas_test() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(0, 0);
    assert_eq!(r.origin, point(0.0, 0.0, 0.0));
    assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
}

#[test]
fn constructing_a_ray_when_the_camera_is_transformed_test() {
    let mut c = Camera::new(201, 101, PI / 2.0);
    c.set_transform(&rotation_y(PI / 4.0) * &translation(0.0, -2.0, 5.0));
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, point(0.0, 2.0, -5.0));
    assert_eq!(r.direction, vector(SQRT2 / 2.0, 0.0, -SQRT2 / 2.0));
}

#[test]
fn rendering_a_world_with_a_camera_test() {
    let w = build_world();
    let mut c = Camera::new(11, 11, PI / 2.0);
    let from = point(0.0, 0.0, -5.0);
    let to = point(0.0, 0.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    c.set_transform(view_transform(from, to, up));
    let image = c.render(&w);
    assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
}

#[test]
fn putting_it_together_test() {
    let mut wall_material = material();
    wall_material.color = Color::new(1.0, 0.9, 0.9);
    wall_material.specular = 0.0;

    let mut floor = build_sphere();
    floor.set_transformation(scaling(10.0, 0.01, 10.0));
    floor.set_material(wall_material.clone());

    let mut left_wall = build_sphere();
    let t = translation(0.0, 0.0, 5.0);
    let ry = rotation_y(-PI / 4.0);
    let rx = rotation_x(PI / 2.0);
    let s = scaling(10.0, 0.01, 10.0);
    let transform = &t * &(&ry * &(&rx * &s));
    left_wall.set_transformation(transform);
    left_wall.set_material(wall_material.clone());

    let mut right_wall = build_sphere();
    right_wall.set_transformation(
        &translation(0.0, 0.0, 5.0) *
            &(&rotation_y(PI / 4.0) * &(&rotation_x(PI / 2.0) *
                &scaling(10.0, 0.01, 10.0))));
    right_wall.set_material(wall_material.clone());

    let mut mid_material = material();
    mid_material.color = Color::new(0.1, 1.0, 0.5);
    mid_material.diffuse = 0.7;
    mid_material.specular = 0.3;
    let mut middle = build_sphere();
    middle.set_transformation(translation(-0.5, 1.0, 0.5));
    middle.set_material(mid_material);

    let mut right_material = material();
    right_material.color = Color::new(0.5, 1.0, 0.1);
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    let mut right = build_sphere();
    right.set_transformation(&translation(1.5, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5));
    right.set_material(right_material);

    let mut left_material = material();
    left_material.color = Color::new(1.0, 0.8, 0.1);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;

    let mut left = build_sphere();
    left.set_transformation(&translation(-1.5, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33));
    left.set_material(left_material);

    let mut world = World::new();
    let lights = vec!(PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0)));
    world.set_lights( lights );
    world.set_objects(vec![floor, left, left_wall, middle, right, right_wall]);

    let mut camera = Camera::new(400, 200, PI/3.0);
    camera.set_transform( view_transform(point(0.0, 1.5, -5.0),
                                      point(0.0, 1.0, 0.0),
                                      vector(0.0, 1.0, 0.0)));

    let canvas = camera.render(&world);
    let result = canvas.save("e:\\tmp\\first_scene.ppm");
    match result {
        Ok(_) => { print!("Ok")}
        Err(error) => { print!("Error: {}", error)}
    }
}
