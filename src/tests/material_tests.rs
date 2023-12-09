#[cfg(test)]
mod material_tests {
    use crate::canvas::Canvas;
    use crate::colors::Color;
    use crate::light::{Light, PointLight};
    use crate::material::material;
    use crate::math::{Float, SQRT2};
    use crate::object::build_sphere;
    use crate::ray::ray;
    use crate::tuple::{point, vector};

    #[test]
    fn default_material_test() {
        let default_material = material();
        assert_eq!(default_material.color, Color { r: 1.0, g: 1.0, b: 1.0 });
        assert_eq!(default_material.ambient, 0.1);
        assert_eq!(default_material.diffuse, 0.9);
        assert_eq!(default_material.specular, 0.9);
        assert_eq!(default_material.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_test() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light: Light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees_test() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, SQRT2 / 2.0, -SQRT2 / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees_test() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector_test() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, -SQRT2 / 2.0, -SQRT2 / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface_test() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }


    #[test]
    fn putting_it_together_test() {
        let mut m = material();
        m.color = Color::new(1.0, 0.2, 1.0);

        let light = PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        // start the ray at z = -5
        let ray_origin = point(0.0, 0.0, -5.0);
        //  put the wall at z = 10
        let wall_z = 10.0;
        let wall_size = 7.0;
        let canvas_pixels = 300;
        let pixel_size = wall_size / canvas_pixels as Float;
        let half = wall_size / 2.0;
        let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
        let mut shape = build_sphere();
        shape.set_material(m);

        // for each row of pixels in the canvas
        for y in 0 .. canvas_pixels - 1 {
            // compute the world y coordinate (top = +half, bottom = -half)
            let world_y = half - pixel_size * y as Float;
            // for each pixel in the row
            for x in 0..canvas_pixels - 1 {
                // compute the world x coordinate(left = -half, right = half)
                let world_x= -half + pixel_size * x  as Float;
                // describe the point on the wall that the ray will target
                let position = point(world_x, world_y, wall_z);
                let r = ray(ray_origin, (position - ray_origin).normalize());
                let xs = shape.intersect(&r);
                match xs.hit() {
                    None => { /* no intersection, do nothing */}
                    Some(hit) => {
                        let point = r.position(hit.t);
                        let normal = hit.shape.normal_at(point);
                        let eye = - r.direction;
                        let color = m.lighting(&light, point, eye, normal);
                        canvas.write_pixel( x, y, color);
                    }
                }
            }
        }

        let result = canvas.save("e:\\tmp\\phong.ppm");
        match result {
            Ok(_) => { print!("Ok") }
            Err(error) => { print!("Error: {}", error) }
        }
    }
}