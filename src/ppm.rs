use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Pixel(pub [u8; 3]);
impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel([r, g, b])
    }

    fn as_hex(&self) -> String {
        format!("{:x?}", self.0)
    }
}

impl From<Pixel> for u32 {
    fn from(value: Pixel) -> Self {
        let [r, g, b] = value.0;
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }
}

pub struct Ppm6 {
    pub pixels: Vec<Pixel>,
    pub width: u32,
    pub height: u32,
}

impl Ppm6 {
    pub fn new(width: u32, height: u32) -> Self {
        Ppm6 {
            width,
            height,
            pixels: Vec::with_capacity(width as usize * height as usize),
        }
    }
}

impl Display for Ppm6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;

        for pixel in &self.pixels {
            let [r, g, b] = pixel.0;
            writeln!(f, "{r} {g} {b}")?;
        }

        Ok(())
    }
}
