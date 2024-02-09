mod scenes;
mod ray_tracer;
mod scene;
mod tests;

use std::env;
use std::process::exit;
use scenes::clover_scene::CloverScene;
use crate::scene::Scene;
use crate::scenes::basic_refraction_scene::BasicRefractionScene;
use crate::scenes::clover_triangles_scene::CloverTriangleScene;
use crate::scenes::cone_scene::ConeScene;
use crate::scenes::cube_scene::CubeScene;
use crate::scenes::cylinder_scene::CylinderScene;
use crate::scenes::group_scene::GroupScene;
use crate::scenes::patterns_scene::PatternsScene;
use crate::scenes::refraction_sphere_scene::RefractionSphereScene;
use crate::scenes::stripe_pattern_scene::StripePatternScene;
use crate::scenes::teapot_scene::TeaPotScene;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("all")) {
        render_all();
        exit(0);
    }

    println!("Start...");
    let scene = TeaPotScene { };
    scene.render(800, 600, "e:\\tmp\\tea_pot.png");
    println!("Done.")
}

fn render_all() {
    render(&CloverScene::new(0.0, 0.0, -9.0), 800, 600, "e:\\tmp\\clover.png");
    render(&CloverTriangleScene::new(0.0, 0.0, -9.0),800, 600, "e:\\tmp\\clover_triangle.png");
    render(&CylinderScene{},400, 400, "e:\\tmp\\cylinders_scene.png");
    render(&ConeScene{},400, 400, "e:\\tmp\\cones_scene.png");
    render(&CubeScene{},400, 400, "e:\\tmp\\cubes_scene.png");
    render(&GroupScene{},400, 400, "e:\\tmp\\group_scene.png");
    render(&PatternsScene{},400, 400, "e:\\tmp\\all_patterns_scene.png");
    render(&StripePatternScene{},400, 400, "e:\\tmp\\pattern_stripe_scene.png");
    render(&BasicRefractionScene{},640, 400, "e:\\tmp\\basic_refraction_sphere_scene.png");
    render(&RefractionSphereScene{},400, 400, "e:\\tmp\\refraction_sphere_scene.png");
}

fn render(scene : &dyn Scene, h_size : usize, v_size : usize, file_path: &str) {
    scene.render(h_size, v_size, file_path);
    let image = image::open(file_path).expect("Failed to open image");
    let ascii_art = artem::convert(image, &artem::config::ConfigBuilder::new().build());
    println!("{}", ascii_art);
}