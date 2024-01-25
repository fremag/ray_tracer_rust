
mod core;
mod ray_tracer;
mod tests;
mod shapes;
mod patterns;
mod lights;
mod colors;
mod canvas;
mod object;
mod material;
mod world;
mod camera;
mod scenes;
mod scene;

use scenes::clover_scene::CloverScene;
use scene::Scene;

fn main() {
    println!("Start...");
    let scene = CloverScene::new(0.0, 0.0, -9.0);
    scene.render(800, 600, "e:\\tmp\\clover.ppm");
    println!("Done.")
}
