use raytracer::ppm::{Pixel, PpmImage};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ppm = PpmImage::new(3, 2);
    
    *ppm.get_mut_pixel(0, 0).unwrap() = Pixel::new(255, 0, 0);
    *ppm.get_mut_pixel(1, 0).unwrap() = Pixel::new(0, 255, 0);
    *ppm.get_mut_pixel(2, 0).unwrap() = Pixel::new(0, 0, 255);
    *ppm.get_mut_pixel(0, 1).unwrap() = Pixel::new(255, 255, 0);
    *ppm.get_mut_pixel(1, 1).unwrap() = Pixel::new(255, 255, 255);
    *ppm.get_mut_pixel(2, 1).unwrap() = Pixel::new(0, 0, 0);

    std::fs::write("artifacts/minimal.ppm", ppm.to_string().as_bytes())?;

    Ok(())
}