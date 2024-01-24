use crate::math::{Float, PI};
use crate::ray_tracer::{build_mesh, curve_sweep_mesh, init_camera, init_world, render};
use crate::tuple::{point, Tuple};

mod ray_tracer;
mod tests;
mod shapes;
mod patterns;
mod lights;
mod math;
mod tuple;
mod projectile;
mod colors;
mod canvas;
mod matrix;
mod transform;
mod ray;
mod intersection;
mod intersections;
mod object;
mod material;
mod world;
mod comps;
mod camera;
mod bounds;

fn main() {
    println!("Start...");
    const R1: Float = 1.0;
    const R2: Float = 0.25;
    let mut world = init_world();
    fn path_clover(t: Float) -> Tuple {
        let x = R1 * ((2.0 * PI * t).cos() + 2.0 * (2.0 * PI * 2.0 * t).cos());
        let y = R1 * ((2.0 * PI * t).sin() - 2.0 * (2.0 * PI * 2.0 * t).sin());
        let z = 2.0 * R2 * (2.0 * PI * 3.0 * t).sin();
        point(x, y, z)
    }

    fn curve_circle(_u: Float, v: Float) -> (Float, Float) {
        let x = R2 * (2.0 * PI * v).cos();
        let y = R2 * (2.0 * PI * v).sin();
        (x, y)
    }

    let mesh = curve_sweep_mesh(80, 8, path_clover, curve_circle);
    let mesh_obj = build_mesh(&mesh, 0.02, true, true);
    world.objects.push(mesh_obj);
    let camera = init_camera(800, 600, 0.0, 0.0, -9.0);
    render(camera, &world, "e:\\tmp\\clover.ppm");
    println!("Done.")
}
