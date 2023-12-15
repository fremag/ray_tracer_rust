#[cfg(test)]
mod world_test {
    use crate::colors::Color;
    use crate::comps::prepare_computations;
    use crate::intersection::Intersection;
    use crate::light::PointLight;
    use crate::material::Material;
    use crate::object::build_sphere;
    use crate::ray::ray;
    use crate::transform::scaling;
    use crate::tuple::{point, vector};
    use crate::world::build_world;

    #[test]
    fn the_default_world_test() {
        let mut sphere_1 = build_sphere();
        sphere_1.set_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
            ambient: 0.1,
        });

        let mut sphere_2 = build_sphere();
        sphere_2.set_transformation(scaling(0.5, 0.5, 0.5));

        let light = PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let world = build_world();
        assert!(world.lights.contains(&light));
    }

    #[test]
    fn intersect_a_world_with_a_ray_test() {
        let w = build_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = w.intersect_world(&r);
        assert_eq!(xs.count(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection_test() {
        let w = build_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection { t: 4.0, object: shape };
        let comps = prepare_computations(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside_test() {
        let mut w = build_world();
        let light = PointLight::new(point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        w.set_lights(vec!(light));
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection { t: 0.5, object: shape };
        let comps = prepare_computations(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses_test() {
        let w = build_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_color_when_a_ray_hits_test() {
        let w = build_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray_test() {
        let mut w = build_world();
        let outer = &mut w.objects[0];
        let mut outer_material = outer.material().clone();
        outer_material.set_ambient(1.0);
        outer.set_material(outer_material);

        let inner = &mut w.objects[1];
        let mut inner_material = inner.material().clone();
        inner_material.set_ambient(1.0);
        inner.set_material(inner_material);

        let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        print!("hello");
        let c = w.color_at(&r);

        assert_eq!(c, inner_material.color);
    }
}