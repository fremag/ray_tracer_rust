use ray_tracer_lib::camera::Camera;
use ray_tracer_lib::colors::Color;
use ray_tracer_lib::core::math::{Float, PI};
use ray_tracer_lib::core::matrix::Matrix;
use ray_tracer_lib::core::transform::{rotation_x, rotation_y, scaling, translation, view_transform};
use ray_tracer_lib::core::tuple::{point, vector};
use ray_tracer_lib::lights::point_light::PointLight;
use ray_tracer_lib::material::Material;
use ray_tracer_lib::object::{build_cube, build_plane, Object};
use ray_tracer_lib::patterns::pattern::Pattern;
use ray_tracer_lib::shapes::csg::{Csg, CsgOperation};
use ray_tracer_lib::shapes::group::Group;
use ray_tracer_lib::world::World;
use crate::scene::Scene;

pub struct MengerSpongeScene {}

impl Scene for MengerSpongeScene {
    fn get_world(&self) -> World {
        let mut world = World::new();
        let lights = vec!(
            PointLight::new(point(-1.0, 5.0, -5.0), Color::new(1.0/2.0, 1.0/2.0, 1.0/2.0)),
            PointLight::new(point(-5.0, 5.0, 1.0), Color::new(1.0/4.0, 1.0/4.0, 1.0/4.0, )),
            PointLight::new(point(5.0, 5.0, -1.0), Color::new(1.0/6.0, 1.0/6.0, 1.0/6.0 )),
        );
        world.set_lights(lights);

        let mut material_floor = Material::new();
        material_floor.pattern = Pattern::checker(Color::white(), Color::black());

        let mut floor = build_plane();
        floor.set_material(material_floor.clone());
        world.objects.push(floor);

        let mut menger_sponge = MengerSponge::new(2);
        menger_sponge.set_transformation(translation(0.0, 1.5, 0.0));

        world.objects.push(menger_sponge);

        world
    }

    fn get_camera(&self, h_size: usize, v_size: usize) -> Camera {
        let mut camera = Camera::new(h_size, v_size, PI / 3.0);
        camera.set_transform(view_transform(point(-1.0, 4.0, -6.0),
                                            point(0.0, 2.5, 0.0),
                                            vector(0.0, 2.0, 0.0)));
        camera
    }
}
pub struct MengerCastleScene {}

impl Scene for MengerCastleScene {
    fn get_world(&self) -> World {
        let mut world = World::new();
        let lights = vec!(
            PointLight::new(point(-1.0, 6.0, -5.0), Color::new(1.0/2.0, 1.0/2.0, 1.0/2.0)),
            PointLight::new(point(-5.0, 6.0, -1.0), Color::new(1.0/4.0, 1.0/4.0, 1.0/4.0, )),
            PointLight::new(point(5.0, 6.0, -1.0), Color::new(1.0/6.0, 1.0/6.0, 1.0/6.0 )),
        );
        world.set_lights(lights);

        let mut material_floor = Material::new();
        material_floor.pattern = Pattern::checker(Color::white(), Color::black());

        let mut floor = build_plane();
        floor.set_material(material_floor.clone());
        world.objects.push(floor);

        let menger_sponge = MengerSponge::new(3);
        let mut cube = build_cube();
        cube.set_transformation(&scaling(2.0, 1.0, 2.0) * &translation(0.0, 1.0, 0.0));

        let mut menger_castle = Object::new_csg(Csg::new(CsgOperation::Difference, menger_sponge, cube));
        menger_castle.set_transformation(translation(0.0, 1.5, 0.00));
        world.objects.push(menger_castle);

        world
    }

    fn get_camera(&self, h_size: usize, v_size: usize) -> Camera {
        let mut camera = Camera::new(h_size, v_size, PI / 3.0);
        camera.set_transform(view_transform(point(2.0, 3.5, -2.0),
                                            point(0.0, 0.0, 0.0),
                                            vector(0.0, 1.0, 0.0)));
        camera
    }
}

pub struct MengerSponge
{}

impl MengerSponge {
    pub fn new(m: usize) -> Object {
        let thing1 = Self::make_cubes(-1.5, -1.5, 1.5, 1.5, 0, m);
        let mut thing2 = Self::make_cubes(-1.5, -1.5, 1.5, 1.5, 0, m);
        let mut thing3 = Self::make_cubes(-1.5, -1.5, 1.5, 1.5, 0, m);

        thing2.set_transformation(rotation_x(PI / 2.0));
        thing3.set_transformation(rotation_y(PI / 2.0));

        let u1 = Object::new_csg(Csg::new(CsgOperation::Union, thing3, thing1));
        let u2 = Object::new_csg(Csg::new(CsgOperation::Union, u1, thing2));

        //
        let mut cube = build_cube();
        cube.set_transformation(scaling(1.5, 1.5, 1.5));
        //
        let sponge = Object::new_csg(Csg::new(CsgOperation::Difference, cube, u2));
        sponge
    }

    fn make_cubes(x1: Float, y1: Float, x2: Float, y2: Float, n: usize, max: usize) -> Object
    {
        let delta_x = (x2 - x1) / 3.0;
        let delta_y = (x2 - x1) / 3.0;

        let sx = delta_x / 2.0;
        let sy = delta_y / 2.0;
        let mut cube = build_cube();
        let tx = x1 + (x2 - x1) / 2.0;
        let ty = y1 + (y2 - y1) / 2.0;
        cube.set_transformation(&translation(tx, ty, 0.0) * &scaling(sx, sy, 2.1));

        if n >= max {
            return cube;
        }

        // left col
        let g1 = Self::make_cubes(x1 + 0.0 * delta_x, y1 + 0.0 * delta_y, x1 + 1.0 * delta_x, y1 + 1.0 * delta_y, n + 1, max);
        let g2 = Self::make_cubes(x1 + 0.0 * delta_x, y1 + 1.0 * delta_y, x1 + 1.0 * delta_x, y1 + 2.0 * delta_y, n + 1, max);
        let g3 = Self::make_cubes(x1 + 0.0 * delta_x, y1 + 2.0 * delta_y, x1 + 1.0 * delta_x, y1 + 3.0 * delta_y, n + 1, max);

        // center col
        let g4 = Self::make_cubes(x1 + 1.0 * delta_x, y1 + 0.0 * delta_y, x1 + 2.0 * delta_x, y1 + 1.0 * delta_y, n + 1, max);
        //var g5 = MakeCubes(x1 + 1*deltaX, y1+1*deltaY, x1 + 2*deltaX, y1 + 2*deltaY, n + 1, max);
        let g6 = Self::make_cubes(x1 + 1.0 * delta_x, y1 + 2.0 * delta_y, x1 + 2.0 * delta_x, y1 + 3.0 * delta_y, n + 1, max);

        // right col
        let g7 = Self::make_cubes(x1 + 2.0 * delta_x, y1 + 0.0 * delta_y, x1 + 3.0 * delta_x, y1 + 1.0 * delta_y, n + 1, max);
        let g8 = Self::make_cubes(x1 + 2.0 * delta_x, y1 + 1.0 * delta_y, x1 + 3.0 * delta_x, y1 + 2.0 * delta_y, n + 1, max);
        let g9 = Self::make_cubes(x1 + 2.0 * delta_x, y1 + 2.0 * delta_y, x1 + 3.0 * delta_x, y1 + 3.0 * delta_y, n + 1, max);

        let group = Group::from(vec![cube, g1, g2, g3, g4, g6, g7, g8, g9], Matrix::<4>::identity());

        return group;
    }
}
