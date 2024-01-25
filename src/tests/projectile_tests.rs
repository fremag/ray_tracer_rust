#[cfg(test)]
pub mod tests {
    use crate::core::math::Float;
    use crate::core::tuple::{point, Tuple, vector};

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
}