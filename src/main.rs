use crate::ppm::{Pixel, Ppm6};
use std::error::Error;

mod ppm;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ppm = Ppm6::new(3, 2);
    ppm.pixels.push(Pixel::new(255, 0, 0));
    ppm.pixels.push(Pixel::new(0, 255, 0));
    ppm.pixels.push(Pixel::new(0, 0, 255));
    ppm.pixels.push(Pixel::new(255, 255, 0));
    ppm.pixels.push(Pixel::new(255, 255, 255));
    ppm.pixels.push(Pixel::new(0, 0, 0));

    std::fs::write("minimal.ppm", ppm.to_string().as_bytes())?;

    Ok(())
}
