#[cfg(test)]
mod tests {
    use crate::colors::Color;
    use crate::core::comps::prepare_computations;
    use crate::core::intersection::Intersection;
    use crate::core::intersections::intersections;
    use crate::lights::point_light::PointLight;
    use crate::material::Material;
    use crate::core::math::{SQRT2};
    use crate::object::{build_plane, build_sphere};
    use crate::patterns::pattern::{Pattern};
    use crate::core::ray::ray;
    use crate::core::transform::{scaling, translation};
    use crate::core::tuple::{point, vector};
    use crate::tests::helper::tests::build_world;
    use crate::world::World;

    #[test]
    fn the_default_world_test() {
        let mut sphere_1 = build_sphere();
        sphere_1.set_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
            ambient: 0.1,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: Pattern::new(),
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
        let shape = w.objects[0].clone();
        let i = Intersection::new(4.0, shape);
        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside_test() {
        let mut w = build_world();
        let light = PointLight::new(point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        w.set_lights(vec!(light));
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[1].clone();
        let i = Intersection::new(0.5,shape);
        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses_test() {
        let w = build_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r, 1);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_color_when_a_ray_hits_test() {
        let w = build_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r, 1);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray_test() {
        let mut w = build_world();
        let outer = &mut w.objects[0];
        let mut outer_material = outer.material().clone();
        outer_material.ambient = 1.0;
        outer.set_material(outer_material);

        let inner = &mut w.objects[1];
        let mut inner_material = inner.material().clone();
        inner_material.ambient = 1.0;
        inner.set_material(inner_material);

        let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));

        let c = w.color_at(&r, 1);

        assert_eq!(c, inner_material.color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light_test() {
        let w = build_world();
        let p = point(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(&w.lights[0], p), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light_test() {
        let w = build_world();
        let p = point(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(&w.lights[0], p), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light_test() {
        let w = build_world();
        let p = point(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(&w.lights[0], p), false);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow_test() {
        let mut w = World::new();
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let s1 = build_sphere();

        let mut s2 = build_sphere();
        s2.set_transformation(translation(0.0, 0.0, 10.0));
        w.set_lights(vec!(light));
        w.set_objects(vec!(s1, s2));

        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, w.objects[1].clone());
        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_reflected_color_for_a_non_reflective_material_test() {
        let mut w = build_world();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));

        {
            let object = &mut w.objects[1];
            let mut mat = object.material().clone();
            mat.ambient = 1.0;
            object.set_material(mat);
        }

        let i = Intersection::new(1.0,w.objects[1].clone());
        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        let color = w.reflected_color(&comps, 1);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material_test() {
        let mut w = build_world();
        let mut shape = build_plane();
        let mut material = Material::new();
        material.reflective = 0.5;
        shape.set_transformation(translation(0.0, -1.0, 0.0));
        shape.set_material(material);

        w.objects.push(shape);
        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -SQRT2 / 2.0, SQRT2 / 2.0));
        let i = Intersection::new(SQRT2,w.objects[2].clone());
        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        let color = w.reflected_color(&comps, 1);
        assert_eq!(color, Color::new(0.19032, 0.2379, 0.14274));
    }

    #[test]
    fn shade_hit_with_a_reflective_material_test() {
        let mut w = build_world();
        let mut shape = build_plane();
        let mut material = Material::new();
        material.reflective = 0.5;
        shape.set_transformation(translation(0.0, -1.0, 0.0));
        shape.set_material(material);

        w.objects.push(shape);

        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -SQRT2 / 2.0, SQRT2 / 2.0));
        let i = Intersection::new(SQRT2,w.objects[2].clone());
        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        let color = w.shade_hit(&comps, 1);
        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces_test() {
        let mut w = build_world();
        w.lights.clear();
        w.objects.clear();

        w.lights.push(PointLight::new(point(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0)));
        let mut lower_plane = build_plane();
        let mut lower_material = Material::new();
        lower_material.reflective = 1.0;
        lower_plane.set_transformation(translation(0.0, -1.0, 0.0));
        lower_plane.set_material(lower_material);
        w.objects.push(lower_plane);

        let mut upper_plane = build_plane();
        let mut upper_material = Material::new();
        upper_material.reflective = 1.0;
        upper_plane.set_transformation(translation(0.0, 1.0, 0.0));
        upper_plane.set_material(upper_material);
        w.objects.push(upper_plane);

        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r, 5);
        assert_ne!(c, Color::black())
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth_test() {
        let mut w = build_world();
        let mut shape = build_plane();
        let mut material = Material::new();
        material.reflective = 0.5;
        shape.set_material(material);

        shape.set_transformation(translation(0.0, -1.0, 0.0));
        w.objects.clear();
        w.objects.push(shape);

        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -SQRT2 / 2.0, SQRT2 / 2.0));

        let i = Intersection::new(SQRT2,w.objects[0].clone());

        let comps = prepare_computations(&i, &r, &intersections(vec!(i.clone())));
        let color = w.reflected_color(&comps, 0);
        assert_eq!(color, Color::black());
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface_test() {
        let w = build_world();
        let shape = &w.objects[0];
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let i1 = Intersection::new(4.0,shape.clone());
        let i2 = Intersection::new(6.0,shape.clone());
        let xs = intersections(vec!(i1, i2));
        let comps = prepare_computations(&xs.intersections[0], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth_test() {
        let mut w = build_world();
        w.objects.clear();
        let mut shape = build_sphere();
        let mut mat = shape.material().clone();
        mat.transparency = 1.0;
        mat.refractive_index = 1.5;
        shape.set_material(mat);

        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let i1 = Intersection::new(4.0,shape.clone());
        let i2 = Intersection::new(6.0,shape.clone());
        let xs = intersections(vec!(i1, i2));

        let comps = prepare_computations(&xs.intersections[0], &r, &xs);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection_test() {
        let mut w = build_world();
        w.objects.clear();
        let mut shape = build_sphere();
        let mut mat = shape.material().clone();
        mat.transparency = 1.0;
        mat.refractive_index = 1.5;
        shape.set_material(mat);

        let r = ray(point(0.0, 0.0, SQRT2 / 2.0), vector(0.0, 1.0, 0.0));
        let i1 = Intersection::new(-SQRT2 / 2.0,shape.clone());
        let i2 = Intersection::new(SQRT2 / 2.0,shape.clone());
        let xs = intersections(vec!(i1, i2));

        // NOTE: this time you're inside the sphere, so you need
        // to look at the second intersection, xs[1], not xs[0]
        let comps = prepare_computations(&xs.intersections[1], &r, &xs);

        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray_test() {
        let mut w = build_world();

        let shape_a = &mut w.objects[0];
        let mut mat_a = shape_a.material().clone();
        mat_a.ambient = 1.0;
        mat_a.pattern = Pattern::test();
        shape_a.set_material(mat_a);

        let shape_b = &mut w.objects[1];
        let mut mat_b = shape_b.material().clone();
        mat_b.transparency = 1.0;
        mat_b.refractive_index = 1.5;
        shape_b.set_material(mat_b);

        let r = ray(point(0.0, 0.0, 0.1), vector(0.0, 1.0, 0.0));
        let i1 = Intersection::new(-0.9899,w.objects[0].clone());
        let i2 = Intersection::new(-0.4899,w.objects[1].clone());
        let i3 = Intersection::new(0.4899,w.objects[1].clone());
        let i4 = Intersection::new(0.9899,w.objects[0].clone());
        let xs = intersections(vec!(i1, i2, i3, i4));
        let comps = prepare_computations(&xs.intersections[2], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::new(0.0, 0.99888, 0.04725));
    }

    #[test]
    fn shade_hit_with_a_transparent_material_test() {
        let mut w = build_world();
        w.objects.clear();

        let mut floor = build_plane();
        floor.set_transformation(translation(0.0, -1.0, 0.0));
        let mut mat = floor.material().clone();
        mat.transparency = 0.5;
        mat.refractive_index = 1.5;
        floor.set_material(mat);
        w.objects.push(floor);

        let mut ball = build_sphere();
        ball.set_transformation(translation(0.0, -3.5, -0.5));
        let mut mat = ball.material().clone();

        mat.color = Color::new(1.0, 0.0, 0.0);
        mat.ambient = 0.5;
        ball.set_material(mat);

        w.objects.push(ball);

        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -SQRT2 / 2.0, SQRT2 / 2.0));
        let xs = intersections(vec!(Intersection::new(SQRT2,w.objects[0].clone())));

        let comps = prepare_computations(&xs[0], &r, &xs);
        let color = w.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material_test() {
        let mut w = build_world();

        let mut floor = build_plane();
        floor.set_transformation(translation(0.0, -1.0, 0.0));
        let mut mat = floor.material().clone();
        mat.reflective = 0.5;
        mat.transparency = 0.5;
        mat.refractive_index = 1.5;
        floor.set_material(mat);
        w.objects.push(floor);

        let mut ball = build_sphere();
        ball.set_transformation(translation(0.0, -3.5, -0.5));

        let mut mat = ball.material().clone();
        mat.color = Color::new(1.0, 0.0, 0.0);
        mat.ambient = 0.5;
        ball.set_material(mat);

        w.objects.push(ball);

        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -SQRT2 / 2.0, SQRT2 / 2.0));
        let xs = intersections(vec!(Intersection::new(SQRT2, w.objects[2].clone())));

        let comps = prepare_computations(&xs[0], &r, &xs);
        let color = w.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}