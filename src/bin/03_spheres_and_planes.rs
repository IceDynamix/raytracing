use raytracer::math::Vector3;
use raytracer::ppm::{Pixel, PpmImage};
use raytracer::raytracing::{Camera, InfinitePlane, SceneObject, Sphere};
use std::error::Error;

struct Scene {
    pub camera: Camera,
    pub scene_objects: Vec<Box<dyn SceneObject>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let scene = Scene {
        camera: Camera {
            position: Vector3::new(0., 0., -30.),
            up: Vector3::new(0., 1., 0.),
            right: Vector3::new(1., 0., 0.),
            forward: Vector3::new(0., 0., 1.),
            focal_distance: 10.,
            screen_width: 64,
            screen_height: 48,
        },

        scene_objects: vec![
            Box::new(Sphere {
                position: Vector3::new(0., 0., 0.),
                radius: 15.,
                material: Pixel::WHITE,
            }),
            Box::new(Sphere {
                position: Vector3::new(-10., 0., -12.5),
                radius: 5.,
                material: Pixel::new(0, 255, 0),
            }),
            Box::new(InfinitePlane {
                normal: Vector3::new(0., 1., 0.),
                offset: -10.,
                material: Pixel::new(128, 128, 128),
            }),
        ],
    };

    let mut image = PpmImage::new(scene.camera.screen_width, scene.camera.screen_height);

    for x in 0..scene.camera.screen_width {
        for y in 0..scene.camera.screen_height {
            *image.get_mut_pixel(x, y).unwrap() = compute_pixel(&scene, x, y);
        }
    }

    std::fs::write("artifacts/sphere.ppm", image.to_string())?;

    Ok(())
}

fn compute_pixel(scene: &Scene, x: usize, y: usize) -> Pixel {
    let camera = &scene.camera;

    let screen_center = camera.position + camera.forward.scale(camera.focal_distance);
    let relative_from_center = camera
        .right
        .scale(x as f64 - camera.screen_width as f64 / 2.)
        + camera.up.scale(y as f64 - camera.screen_height as f64 / 2.);

    let pixel_vector = screen_center + relative_from_center;
    let ray = (pixel_vector - camera.position).normalized();

    let mut closest_obj = None;
    let mut min_distance = f64::MAX;

    for obj in &scene.scene_objects {
        if let Some(distance) = obj.intersects_ray(camera, &ray) {
            if distance < min_distance {
                closest_obj = Some(obj);
                min_distance = distance;
            }
        }
    }

    match closest_obj {
        Some(obj) => {
            // intersection found
            let light_falloff = 30.;
            let lum = (1. - min_distance * min_distance / (light_falloff * light_falloff)).max(0.);
            let material = obj.material();
            Pixel::new(
                (material.r as f64 * lum) as u8,
                (material.g as f64 * lum) as u8,
                (material.b as f64 * lum) as u8,
            )
        }
        None => Pixel::BLACK,
    }
}
