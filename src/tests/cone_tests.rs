use rand::{Rng, SeedableRng};
use crate::camera::Camera;
use crate::colors::Color;
#[cfg(test)]
use crate::cone::Cone;
use crate::light::PointLight;
use crate::material::Material;
use crate::math::{equals, Float, PI, SQRT2};
use crate::object::build_cone;
use crate::ray::ray;
use crate::transform::{scaling, translation, view_transform};
use crate::tuple::{point, Tuple, vector};
use crate::world::World;

fn intersecting_a_cone_with_a_ray(origin: Tuple, direction: Tuple, t0: Float, t1: Float) {
    let shape = Cone::new();
    let direction = direction.normalize();
    let r = ray(origin, direction);
    let xs = shape.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert!(equals(xs[0], t0));
    assert!(equals(xs[1], t1));
}

#[test]
fn intersecting_a_cone_with_a_ray_test() {
    intersecting_a_cone_with_a_ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 5.0, 5.0);
    intersecting_a_cone_with_a_ray(point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0), 8.66025, 8.66025);
    intersecting_a_cone_with_a_ray(point(1.0, 1.0, -5.0), vector(-0.5, -1.0, 1.0), 4.55006, 49.44994);
}

#[test]
fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves_test() {
    let shape = Cone::new();
    let direction = vector(0.0, 1.0, 1.0).normalize();
    let r = ray(point(0.0, 0.0, -1.0), direction);
    let xs = shape.intersect(&r);
    assert_eq!(xs.len(), 1);
    assert!(equals(xs[0], 0.35355));
}

fn intersecting_a_cone_s_end_caps(origin: Tuple, direction: Tuple, count: usize) {
    let shape = Cone::from(-0.5, 0.5, true);
    let direction = direction.normalize();
    let r = ray(origin, direction);
    let xs = shape.intersect(&r);
    assert_eq!(xs.len(), count);
}

#[test]
fn intersecting_a_cone_s_end_caps_tests() {
    intersecting_a_cone_s_end_caps(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0), 0);
    intersecting_a_cone_s_end_caps(point(0.0, 0.0, -0.25), vector(0.0, 1.0, 1.0), 2);
    intersecting_a_cone_s_end_caps(point(0.0, 0.0, -0.25), vector(0.0, 1.0, 0.0), 4);
}

#[test]
fn computing_the_normal_vector_on_a_cone_test() {
    let shape = Cone::new();
    assert_eq!(shape.normal_at(&point(0.0, 0.0, 0.0)), vector(0.0, 0.0, 0.0));
    assert_eq!(shape.normal_at(&point(1.0, 1.0, 1.0)), vector(1.0, -SQRT2, 1.0));
    assert_eq!(shape.normal_at(&point(-1.0, -1.0, 0.0)), vector(-1.0, 1.0, 0.0));
}


#[test]
fn cone_putting_it_together_test() {
    let mut world = World::new();
    let lights = vec!(
        PointLight::new(point(0.0, 50.0, -50.0), Color::white() ),
    );
    world.set_lights(lights);
    const N: i32 = 9;
    let h_n = N as Float / 2.0;
    let mut mat = Material::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(2);

    for i in 0..N {
        for j in 0..N {
            let s_y = rng.gen::<Float>() *0.8+ 0.2;
            let mut cone = build_cone(-s_y, 0.0);
            let t_x = (i as Float - h_n) / 2.0;
            let t_z = (j as Float - h_n) / 2.0;

            let translation = translation(t_x, s_y, t_z);
            let scaling = scaling(0.25, s_y, 0.25);
            mat.color = Color::new(rng.gen(), rng.gen(), rng.gen());
            mat.reflective = 0.3+0.5 * rng.gen::<Float>();

            let matrix = &translation * &scaling;
            cone.set_transformation(matrix);
            cone.set_material(mat);
            world.objects.push(cone);
        }
    }

    let mut camera = Camera::new(400, 400, PI / 3.0);
    camera.set_transform(view_transform(point(-0.5, 4.0, -2.0),
                                        point(0.0, 0.0, 0.0),
                                        vector(0.0, 1.0, 0.0)));

    let canvas = camera.render(&world);
    let result = canvas.save("e:\\tmp\\cones_scene.ppm");
    match result {
        Ok(_) => { print!("Ok") }
        Err(error) => { print!("Error: {}", error) }
    }
}
