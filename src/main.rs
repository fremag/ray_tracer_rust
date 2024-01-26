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

use std::env;
use std::process::exit;
use scenes::clover_scene::CloverScene;
use scene::Scene;
use crate::scenes::basic_refraction_scene::BasicRefractionScene;
use crate::scenes::cone_scene::ConeScene;
use crate::scenes::cube_scene::CubeScene;
use crate::scenes::cylinder_scene::CylinderScene;
use crate::scenes::group_scene::GroupScene;
use crate::scenes::patterns_scene::PatternsScene;
use crate::scenes::refraction_sphere_scene::RefractionSphereScene;
use crate::scenes::stripe_pattern_scene::StripePatternScene;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("all")) {
        render_all();
        exit(0);
    }

    println!("Start...");
    let scene = CloverScene::new(0.0, 0.0, -9.0);
    scene.render(800, 600, "e:\\tmp\\clover.ppm");
    println!("Done.")
}

fn render_all() {
    let scene = CloverScene::new(0.0, 0.0, -9.0);
    scene.render(800, 600, "e:\\tmp\\clover.ppm");

    CylinderScene{}.render(400, 400, "e:\\tmp\\cylinders_scene.ppm");
    ConeScene{}.render(400, 400, "e:\\tmp\\cones_scene.ppm");
    CubeScene{}.render(400, 400, "e:\\tmp\\cubes_scene.ppm");
    GroupScene{}.render(400, 400, "e:\\tmp\\group_scene.ppm");
    PatternsScene{}.render(400, 400, "e:\\tmp\\all_patterns_scene.ppm");
    StripePatternScene{}.render(400, 400, "e:\\tmp\\pattern_stripe_scene.ppm");
    BasicRefractionScene{}.render(640, 400, "e:\\tmp\\basic_refraction_sphere_scene.ppm");
    RefractionSphereScene{}.render(400, 400, "e:\\tmp\\refraction_sphere_scene.ppm");
}