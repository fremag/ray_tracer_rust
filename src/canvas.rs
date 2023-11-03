use crate::colors::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width : usize,
    pub height : usize,
    pub pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width : usize, height : usize) -> Self {
        let pixels : Vec<Vec<Color>> = vec![vec![Color::new(0.0, 0.0, 0.0); width]; height];
        Self {width, height, pixels}
    }

    pub fn write_pixel(&mut self, x : usize, y : usize, color : Color) {
        self.pixels[y][x] = color;
    }

    pub fn pixel_at(&self, x : usize, y : usize) -> Color {
        self.pixels[y][x]
    }
}