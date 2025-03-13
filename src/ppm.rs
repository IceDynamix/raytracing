use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }

    #[allow(unused)]
    fn as_hex(&self) -> String {
        format!("{:x?}", [self.r, self.g, self.b])
    }

    pub const WHITE: Pixel = Pixel {
        r: 255,
        g: 255,
        b: 255,
    };

    pub const BLACK: Pixel = Pixel { r: 0, g: 0, b: 0 };
}

impl From<Pixel> for u32 {
    fn from(value: Pixel) -> Self {
        ((value.r as u32) << 16) | ((value.g as u32) << 8) | (value.b as u32)
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel::BLACK
    }
}

pub struct PpmP3 {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Pixel>,
}

impl PpmP3 {
    pub fn new(width: usize, height: usize) -> Self {
        PpmP3 {
            width,
            height,
            pixels: vec![Pixel::default(); width * height],
        }
    }

    pub fn get_mut_pixel(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        self.pixels.get_mut(y * self.width + x)
    }
}

impl Display for PpmP3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;

        for pixel in &self.pixels {
            writeln!(f, "{} {} {}", pixel.r, pixel.g, pixel.b)?;
        }

        Ok(())
    }
}
