#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::colors::Color;
    use crate::projectile::{Environment, Projectile};
    use crate::tuple::{point, vector};

    #[test]
    fn creating_a_canvas_test() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        let black = Color::new(0.0, 0.0, 0.0);

        for y in 0..canvas.height {
            for x in 0..canvas.width {
                assert_eq!(canvas.pixel_at(x, y), black)
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas_test() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);
        let pixel_color = canvas.pixel_at(2, 3);
        assert_eq!(pixel_color, red)
    }

    #[test]
    fn construction_ppm_header_test() {
        let canvas = Canvas::new(5, 3);
        let header = canvas.ppm_header();
        assert_eq!(header, "P3\n5 3\n255\n")
    }

    #[test]
    fn constructing_pixel_data_test() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);

        let pixel_data = canvas.pixel_data();
        assert_eq!(pixel_data, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n")
    }

    #[test]
    fn splitting_long_lines_ppm_test() {
        let mut canvas = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for y in 0..2 {
            for x in 0..10 {
                canvas.write_pixel(x, y, color);
            }
        }

        let pixel_data = canvas.pixel_data();
        assert_eq!(pixel_data, "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 \n153 255 204 153 255 204 153 255 204 153 255 204 153 \n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 \n153 255 204 153 255 204 153 255 204 153 255 204 153 \n")
    }

    #[test]
    fn putting_it_together_test() {
        let start = point(0.0, 1.0, 0.0);
        let velocity = vector(1.0, 1.8, 0.0).normalize() * 11.25;
        let mut p = Projectile::new(start, velocity);
        let gravity = vector(0.0, -0.1, 0.0);
        let wind = vector(-0.01, 0.0, 0.0);
        let e = Environment::new(gravity, wind);
        let red = Color::new(1.0, 0.0, 0.0);

        let mut canvas = Canvas::new(900, 550);
        while p.position.y > 0.0 {
            p = p.tick(&e);
            canvas.write_pixel(p.position.x as usize, canvas.height - p.position.y as usize, red);
        }

        let result = canvas.save("e:\\tmp\\projectile.ppm");
        match result {
            Ok(_) => { print!("Ok") }
            Err(error) => { print!("Error: {}", error) }
        }
    }
}