#[cfg(test)]
pub mod tests {
    use ray_tracer_lib::canvas::Canvas;
    use ray_tracer_lib::colors::Color;
    use ray_tracer_lib::core::math::Float;
    use ray_tracer_lib::core::tuple::{point, Tuple, vector};

    #[test]
    fn projectile_test() {
        let mut proj = Projectile::new(point(0.0, 1.0, 0.0), vector(1.0, 1.0, 0.0));
        let gravity = vector(0.0, -0.1, 0.0);
        let wind = vector(-0.01, 0.0, 0.0);
        let environment = Environment::new(gravity, wind);

        let zero = 0.0 as Float;
        let mut i = 0;
        while proj.position.y > zero && i < 1_000 {
            proj = proj.tick(&environment);
            i += 1;
        }

        assert_eq!(i < 1_000, true)
    }

    pub struct Projectile {
        pub position : Tuple,
        velocity : Tuple
    }

    pub struct Environment {
        gravity : Tuple,
        wind: Tuple
    }

    impl Projectile {
        pub fn new(position : Tuple, velocity : Tuple) -> Projectile {
            Projectile {position, velocity}
        }

        pub fn tick(&self, environment: &Environment) -> Projectile {
            Projectile::new(self.position+self.velocity, self.velocity+environment.gravity+environment.wind)
        }
    }

    impl Environment  {
        pub fn new(gravity : Tuple, wind: Tuple) -> Environment {
            Environment{gravity, wind}
        }
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
            Ok(_) => { println!("Ok") }
            Err(error) => { println!("Error: {}", error) }
        }
    }

}