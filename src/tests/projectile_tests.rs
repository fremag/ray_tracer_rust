#[cfg(test)]
use crate::math::Float;
use crate::projectile::{Environment, Projectile};
use crate::tuple::{point, vector};

#[test]
fn projectile_test() {
    let mut proj = Projectile::new(point(0.0, 1.0, 0.0), vector(1.0, 1.0 , 0.0));
    let gravity = vector(0.0, -0.1, 0.0);
    let wind = vector(-0.01, 0.0, 0.0);
    let environment = Environment::new(gravity, wind);

    let zero = 0.0 as Float;
    let mut i = 0;
    while proj.position.y > zero && i < 1_000{
        proj = proj.tick(&environment);
        i+=1;
    }

    assert_eq!(i < 1_000, true)
}