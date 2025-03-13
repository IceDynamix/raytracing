use raytracer::math::Vector2;
use raytracer::ppm::{Pixel, PpmP3};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 64;
    let height = 48;
    let mut ppm = PpmP3::new(width, height);

    let origin = Vector2::new(width as f64, height as f64).scale(0.5);
    let radius = 5.;

    for x in 0..width {
        for y in 0..height {
            let v = Vector2::new(x as f64, y as f64);
            let from_origin = origin - v;
            let in_circle = from_origin.euclidean_norm() <= radius;

            let pixel = if in_circle {
                println!("{from_origin:?} {}", from_origin.euclidean_norm());
                Pixel::WHITE
            } else {
                Pixel::BLACK
            };

            match ppm.get_mut_pixel(x, y) {
                None => {
                    dbg!(x);
                    dbg!(y);
                }
                Some(p) => {
                    *p = pixel;
                }
            }
        }
    }

    std::fs::write("artifacts/circle.ppm", ppm.to_string().as_bytes())?;

    Ok(())
}
