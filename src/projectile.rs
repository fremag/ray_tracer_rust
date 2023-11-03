use crate::tuple::Tuple;

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