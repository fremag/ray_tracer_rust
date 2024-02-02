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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("all")) {
        render_all();
        exit(0);
    }

    println!("Start...");
    let scene = CloverTriangleScene::new(0.0, 0.0, -9.0);
    scene.render(800, 600, "e:\\tmp\\clover_triangle.png");
    println!("Done.")
}

fn render_all() {
    CloverScene::new(0.0, 0.0, -9.0).render(800, 600, "e:\\tmp\\clover.png");
    CloverTriangleScene::new(0.0, 0.0, -9.0).render(800, 600, "e:\\tmp\\clover_triangle.png");
    CylinderScene{}.render(400, 400, "e:\\tmp\\cylinders_scene.png");
    ConeScene{}.render(400, 400, "e:\\tmp\\cones_scene.png");
    CubeScene{}.render(400, 400, "e:\\tmp\\cubes_scene.png");
    GroupScene{}.render(400, 400, "e:\\tmp\\group_scene.png");
    PatternsScene{}.render(400, 400, "e:\\tmp\\all_patterns_scene.png");
    StripePatternScene{}.render(400, 400, "e:\\tmp\\pattern_stripe_scene.png");
    BasicRefractionScene{}.render(640, 400, "e:\\tmp\\basic_refraction_sphere_scene.png");
    RefractionSphereScene{}.render(400, 400, "e:\\tmp\\refraction_sphere_scene.png");
}