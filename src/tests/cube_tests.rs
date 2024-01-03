use rand::{Rng, SeedableRng};
use crate::camera::Camera;
use crate::colors::Color;
use crate::cube::Cube;
use crate::light::PointLight;
use crate::material::Material;
use crate::math::{Float, PI};
use crate::object::build_cube;
use crate::ray::ray;
use crate::transform::{scaling, translation, view_transform};
use crate::tuple::{point, vector};
use crate::world::World;

#[cfg(test)]
#[test]
fn a_ray_intersects_a_cube_test() {
    a_ray_intersects_a_cube(5.0, 0.5, 0.0, -1.0, 0.0, 0.0, 4.0, 6.0);
    a_ray_intersects_a_cube(-5.0, 0.5, 0.0, 1.0, 0.0, 0.0, 4.0, 6.0);
    a_ray_intersects_a_cube(0.5, 5.0, 0.0, 0.0, -1.0, 0.0, 4.0, 6.0);
    a_ray_intersects_a_cube(0.5, -5.0, 0.0, 0.0, 1.0, 0.0, 4.0, 6.0);
    a_ray_intersects_a_cube(0.5, 0.0, 5.0, 0.0, 0.0, -1.0, 4.0, 6.0);
    a_ray_intersects_a_cube(0.5, 0.0, -5.0, 0.0, 0.0, 1.0, 4.0, 6.0);
    a_ray_intersects_a_cube(0.0, 0.5, 0.0, 0.0, 0.0, 1.0, -1.0, 1.0);
}

fn a_ray_intersects_a_cube(px: Float, py: Float, pz: Float, dx: Float, dy: Float, dz: Float, t1: Float, t2: Float) {
    let c = Cube::new();
    let r = ray(point(px, py, pz), vector(dx, dy, dz));
    let xs = c.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], t1);
    assert_eq!(xs[1], t2);
}

#[test]
fn a_ray_misses_a_cube_test() {
    a_ray_misses_a_cube(-2.0, 0.0, 0.0, 0.2673, 0.5345, 0.8018);
    a_ray_misses_a_cube(0.0, -2.0, 0.0, 0.8018, 0.2673, 0.5345);
    a_ray_misses_a_cube(0.0, 0.0, -2.0, 0.5345, 0.8018, 0.2673);
    a_ray_misses_a_cube(2.0, 0.0, 2.0, 0.0, 0.0, -1.0);
    a_ray_misses_a_cube(0.0, 2.0, 2.0, 0.0, -1.0, 0.0);
    a_ray_misses_a_cube(2.0, 2.0, 0.0, -1.0, 0.0, 0.0);
}

fn a_ray_misses_a_cube(px: Float, py: Float, pz: Float, dx: Float, dy: Float, dz: Float) {
    let c = Cube::new();
    let r = ray(point(px, py, pz), vector(dx, dy, dz));
    let xs = c.intersect(&r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn cube_putting_it_together_test() {
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
            let mut cube = build_cube();
            let t_x = (i as Float - h_n) / 2.0;
            let t_z = (j as Float - h_n) / 2.0;

            let s_y = rng.gen::<Float>() *0.8+ 0.2;
            let t_y = s_y;

            let translation = translation(t_x, t_y, t_z);
            let scaling = scaling(0.25, s_y, 0.25);
            mat.color = Color::new(rng.gen(), rng.gen(), rng.gen());
            mat.reflective = 0.3+0.5 * rng.gen::<Float>();

            let matrix = &translation * &scaling;
            cube.set_transformation(matrix);
            cube.set_material(mat);
            world.objects.push(cube);
        }
    }

    let mut camera = Camera::new(400, 400, PI / 3.0);
    camera.set_transform(view_transform(point(-0.5, 4.0, -2.0),
                                        point(0.0, 0.0, 0.0),
                                        vector(0.0, 1.0, 0.0)));

    let canvas = camera.render(&world);
    let result = canvas.save("e:\\tmp\\cubes_scene.ppm");
    match result {
        Ok(_) => { print!("Ok") }
        Err(error) => { print!("Error: {}", error) }
    }
}
