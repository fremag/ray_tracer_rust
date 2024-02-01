use ray_tracer_lib::camera::Camera;
use ray_tracer_lib::core::math::{Float, PI};
use ray_tracer_lib::core::transform::view_transform;
use ray_tracer_lib::core::tuple::{point, Tuple, vector};
use crate::ray_tracer::{build_mesh, curve_sweep_mesh};
use ray_tracer_lib::world::World;
use crate::scene::Scene;

pub struct CloverScene {
    from_x: Float,
    from_y: Float,
    from_z: Float,
}

impl CloverScene {
    pub fn new(from_x: Float, from_y: Float, from_z: Float
    ) -> Self {
        Self {
            from_x,
            from_y,
            from_z
        }
    }
}

impl Scene for CloverScene {
    fn get_world(&self) -> World {
        const R1: Float = 1.0;
        const R2: Float = 0.25;
        let mut world = Self::init_world(false);
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
        world
    }

    fn get_camera(&self, h_size: usize, v_size: usize) -> Camera {
        let mut camera = Camera::new(h_size, v_size, PI / 3.0);
        camera.set_transform(view_transform(point(self.from_x, self.from_y, self.from_z),
                                            point(0.0, 0.0 ,0.0 ),
                                            vector(0.0, 1.0, 0.0)));
        camera
    }
}
