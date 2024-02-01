use std::fs::File;
use std::io::{Write, Error};
use crate::colors::Color;
use crate::core::math::Float;

#[derive(Debug)]
pub struct Canvas {
    pub width : usize,
    pub height : usize,
    pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width : usize, height : usize) -> Self {
        let pixels : Vec<Vec<Color>> = vec![vec![Color::new(0.0, 0.0, 0.0); width]; height];
        Self {width, height, pixels}
    }

    pub fn write_pixel(&mut self, x : usize, y : usize, color : Color) {
        if x >= self.width {
            return;
        }
        if y >= self.height {
            return;
        }

        self.pixels[y][x] = color;
    }

    pub fn pixel_at(&self, x : usize, y : usize) -> Color {
        self.pixels[y][x]
    }

    pub fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    pub fn pixel_data(&self) -> String {
        let mut data = String::new();
        let mut line = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.pixel_at(x, y);
                self.push(&mut line, &mut data, c.r);
                self.push(&mut line, &mut data, c.g);
                self.push(&mut line, &mut data, c.b);
            }
            data.push_str(&line);
            data.push('\n');
            line.clear()
        }

        data
    }

    fn push(&self, line: &mut String, data: &mut String, value : Float) {
        if line.len() >= 65 {
            data.push_str(line);
            data.push('\n');
            line.clear()
        }

        let mut c = 255.0 * value;
        if c < 0.0 {
            c = 0.0;
        }
        if c > 255.0 {
            c = 255.0;
        }

        let val_str = format!("{:.0}", c);
        line.push_str(&val_str);
        line.push(' ' );
    }

    pub fn save(&self, path: &str) -> Result<(), Error> {
        let mut output = File::create(path)?;
        write!(output, "{}", self.ppm_header())?;
        write!(output, "{}", self.pixel_data())?;

        Ok(())
    }
}

