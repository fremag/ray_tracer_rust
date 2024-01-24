use crate::colors::Color;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct CheckerPattern {
    pub a : Color,
    pub b : Color,
}

impl CheckerPattern {
    pub fn new(a: Color, b: Color) -> Self { Self {a, b} }

    pub(crate) fn pattern_at(&self, point: &Tuple) -> Color {
        let sum_floor_coord = point.x.floor() as i32  + point.y.floor() as i32 + point.z.floor()  as i32;
        if sum_floor_coord % 2 == 0 {
            return self.a
        }

        self.b
    }
}
