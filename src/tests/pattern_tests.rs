use crate::camera::Camera;
#[cfg(test)]
use crate::colors::Color;
use crate::light::PointLight;
use crate::material::Material;
use crate::math::PI;
use crate::matrix::Matrix;
use crate::object::{build_plane, build_sphere};
use crate::pattern::{Pattern, StripePattern};
use crate::transform::{rotation_x, scaling, translation, view_transform};
use crate::tuple::{point, vector};
use crate::world::World;

#[test]
fn creating_a_stripe_pattern_test() {
    let pattern = StripePattern::new(Color::white(), Color::black());
    assert_eq!(pattern.a, Color::white());
    assert_eq!(pattern.b, Color::black());
}

#[test]
fn a_stripe_pattern_is_constant_in_y_test() {
    let pattern = StripePattern::new(Color::white(), Color::black());
    assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 0.0)), Color::white());
    assert_eq!(pattern.stripe_at(&point(0.0, 1.0, 0.0)), Color::white());
    assert_eq!(pattern.stripe_at(&point(0.0, 2.0, 0.0)), Color::white());
}

#[test]
fn a_stripe_pattern_is_constant_in_z_test() {
    let pattern = StripePattern::new(Color::white(), Color::black());
    assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 0.0)), Color::white());
    assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 1.0)), Color::white());
    assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 2.0)), Color::white());
}

#[test]
fn a_stripe_pattern_alternates_in_x_test() {
    let pattern = StripePattern::new(Color::white(), Color::black());
    assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 0.0)), Color::white());
    assert_eq!(pattern.stripe_at(&point(0.9, 0.0, 0.0)), Color::white());
    assert_eq!(pattern.stripe_at(&point(1.0, 0.0, 0.0)), Color::black());
    assert_eq!(pattern.stripe_at(&point(-0.1, 0.0, 0.0)), Color::black());
    assert_eq!(pattern.stripe_at(&point(-1.0, 0.0, 0.0)), Color::black());
    assert_eq!(pattern.stripe_at(&point(-1.1, 0.0, 0.0)), Color::white());
}

#[test]
fn putting_it_together_test() {
    let mut material = Material::new();
    material.color = Color::new(1.0, 0.5, 0.5);
    material.specular = 0.0;
    material.pattern = Pattern::stripe(Color::white(), material.color);

    let mut floor = build_plane();
    floor.set_material(material.clone());

    let mut wall = build_plane();
    wall.set_transformation(&translation(0.0, 0.0, 4.0) * &rotation_x(PI/2.0));
    wall.set_material(material.clone());

    let mut middle = build_sphere();
    middle.set_transformation(translation(-0.5, 1.0, 0.5));
    middle.set_material(material);

    let mut right = build_sphere();
    right.set_transformation(&translation(1.0, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5));
    right.set_material(material);

    let mut left = build_sphere();
    left.set_transformation(&translation(-2.0, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33));
    left.set_material(material);

    let mut world = World::new();
    let lights = vec!(PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0)));
    world.set_lights( lights );
    world.set_objects(vec![
        floor,
        wall,
        left,
        middle,
        right,
    ]);

    let mut camera = Camera::new(400, 200, PI/3.0);
    camera.set_transform( view_transform(point(-2.5, 1.5, -5.0),
                                         point(0.0, 1.0, 0.0),
                                         vector(0.0, 1.0, 0.0)));

    let canvas = camera.render(&world);
    let result = canvas.save("e:\\tmp\\pattern_stripe_scene.ppm");
    match result {
        Ok(_) => { print!("Ok")}
        Err(error) => { print!("Error: {}", error)}
    }
}


#[test]
fn pattern_with_an_object_transformation_test() {
    let mut object = build_sphere();
    object.set_transformation(scaling(2.0, 2.0, 2.0));
    let pattern = Pattern:: stripe(Color::white(), Color::black());
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
