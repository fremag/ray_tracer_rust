mod scenes;
mod ray_tracer;
mod scene;
mod tests;

use std::{env};
use std::process::exit;
use scenes::clover_scene::CloverScene;
use crate::scene::Scene;
use crate::scenes::basic_refraction_scene::BasicRefractionScene;
use crate::scenes::clover_triangles_scene::CloverTriangleScene;
use crate::scenes::cone_scene::ConeScene;
use crate::scenes::csg_scene::CsgScene;
use crate::scenes::cube_scene::CubeScene;
use crate::scenes::cylinder_scene::CylinderScene;
use crate::scenes::dragon_scene::DragonScene;
use crate::scenes::group_scene::GroupScene;
use crate::scenes::menger_scene::{MengerCastleScene, MengerSpongeScene};
use crate::scenes::patterns_scene::PatternsScene;
use crate::scenes::refraction_sphere_scene::RefractionSphereScene;
use crate::scenes::smooth_teapot_scene::SmoothTeaPotScene;
use crate::scenes::stripe_pattern_scene::StripePatternScene;
use crate::scenes::teapot_scene::TeaPotScene;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("all")) {
        render_all();
        exit(0);
    }
    let nb_threads = num_cpus::get() - 1;
    rayon::ThreadPoolBuilder::new().num_threads(nb_threads).build_global().unwrap();

    println!("Start... {} threads", nb_threads);
    render(&MengerCastleScene{},800, 800, "./img/menger_castle_scene.png");
    println!("Done.")
}

fn render_all() {
    render(&CloverScene::new(0.0, 0.0, -9.0), 800, 600, "./img/clover.png");
    render(&CloverTriangleScene::new(0.0, 0.0, -9.0),800, 600, "./img/clover_triangle.png");
    render(&CylinderScene{},400, 400, "./img/cylinders_scene.png");
    render(&ConeScene{},400, 400, "./img/cones_scene.png");
    render(&CubeScene{},400, 400, "./img/cubes_scene.png");
    render(&GroupScene{},400, 400, "./img/group_scene.png");
    render(&PatternsScene{},400, 400, "./img/all_patterns_scene.png");
    render(&StripePatternScene{},400, 400, "./img/pattern_stripe_scene.png");
    render(&BasicRefractionScene{},640, 400, "./img/basic_refraction_sphere_scene.png");
    render(&RefractionSphereScene{},400, 400, "./img/refraction_sphere_scene.png");
    render(&TeaPotScene{},400, 400, "./img/teapot.png");
    render(&DragonScene { }, 800, 600, "./img/dragon.png");
    render(&SmoothTeaPotScene{},400, 400, "./img/teapot_smooth.png");
    render(&CsgScene{},400, 400, "./img/csg_scene.png");
    render(&MengerSpongeScene{},400, 400, "./img/menger_sponge_scene.png");
}

fn render(scene : &dyn Scene, h_size : usize, v_size : usize, file_path: &str) {
    scene.render(h_size, v_size, file_path);
    let image = image::open(file_path).expect("Failed to open image");
    let config = &artem::config::ConfigBuilder::new().border(true).build();
    let ascii_art = artem::convert(image, config);
    println!("{}", ascii_art);
}