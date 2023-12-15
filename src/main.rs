mod ray_tracer;
mod tests;
mod core;
mod math;
mod tuple;
mod projectile;
mod colors;
mod canvas;
mod matrix;
mod transform;
mod ray;
mod sphere;
mod intersection;
mod shape;
mod intersections;
mod object;
mod light;
mod material;
mod world;
mod comps;
mod camera;

fn main() {
    println!("Hello, world!");
    println!("{0}", ray_tracer::add(2, 3))
}

