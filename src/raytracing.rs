use crate::math::Vector3;
use crate::ppm::Pixel;

pub trait SceneObject {
    fn intersects_ray(&self, camera: &Camera, ray: &Vector3) -> Option<f64>;
    fn material(&self) -> &Pixel;
}

#[derive(Debug)]
pub struct Sphere {
    pub position: Vector3,
    pub radius: f64,
    pub material: Pixel,
}

impl SceneObject for Sphere {
    // the definition of a sphere is all points with equal distance from the center
    // `||v - sphere_center|| = sqrt(<v - sphere_center, v - sphere_center>) = radius`
    //
    // the ray cast from the camera is `camera_pos + camera_to_pixel.scale(t)`
    //
    // to check for intersection, we insert the ray into the sphere equation and solve for t
    // ```
    // ||camera_pos + camera_to_pixel.scale(t) - sphere_center|| = radius`
    //
    // <camera_pos + camera_to_pixel.scale(t) - sphere_center> = radius^2`
    //
    // solve for t
    //
    //    t^2 * <camera_to_pixel, camera_to_pixel>
    // +  2t  * <camera_pos - sphere_center, camera_to_pixel>
    // +        <camera_pos - sphere_center, camera_pos - sphere_center>
    // = radius^2
    //
    // use quadratic equation to solve for t:
    //
    // t = (-b +- sqrt(b^2 - 4ac)) / 2a
    //
    // with a = <camera_to_pixel, camera_to_pixel>
    //      b = 2 * <camera_pos - sphere_center, camera_to_pixel>
    //      c = <camera_pos - sphere_center, camera_pos - sphere_center> - radius^2
    //
    // if the discriminant is negative then there is no intersection.
    // ```
    fn intersects_ray(&self, camera: &Camera, ray: &Vector3) -> Option<f64> {
        let ray = ray.normalized();
        let h = camera.position - self.position;

        let a = ray.dot_product(&ray); // ray is normalized, so <ray, ray> == 1
        let b = 2. * h.dot_product(&ray);
        let c = h.dot_product(&h) - self.radius * self.radius;

        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            // No intersection
            None
        } else {
            let intersections = [
                (-b + discriminant.sqrt()) / (2. * a),
                (-b - discriminant.sqrt()) / (2. * a),
            ];

            intersections
                .into_iter()
                .filter(|&t| t >= 0.) // intersection must be in front of the camera
                .min_by(f64::total_cmp) // look for the closest intersection
        }
    }

    fn material(&self) -> &Pixel {
        &self.material
    }
}

#[derive(Debug)]
pub struct InfinitePlane {
    pub normal: Vector3,
    pub offset: f64,
    pub material: Pixel,
}

impl SceneObject for InfinitePlane {
    // a plane is defined by the equation <v, normal> = d
    // once again we want to insert the ray equation and solve for t
    //
    // d = <v, normal>
    //   = <c + dt, normal>
    //   = <c, normal> + <dt, normal>
    //   = <c, normal> + t <d, normal>
    //
    // d - <c, normal> = t <d, normal>
    //
    // (d - <c, normal>) / <d, normal> = t
    //
    // if <d, normal> = 0 (the ray is parallel to the plane) then
    // there is no intersection (and we ignore the case where the ray is in the plane)
    fn intersects_ray(&self, camera: &Camera, ray: &Vector3) -> Option<f64> {
        let angle = ray.dot_product(&self.normal);

        if angle == 0. {
            None
        } else {
            let distance = (-self.offset - camera.position.dot_product(&self.normal)) / angle;
            if distance < 0. {
                None
            } else {
                Some(distance)
            }
        }
    }

    fn material(&self) -> &Pixel {
        &self.material
    }
}

#[derive(Debug)]
pub struct Camera {
    pub position: Vector3,
    pub up: Vector3,
    pub right: Vector3,
    pub forward: Vector3,
    pub focal_distance: f64,
    pub screen_width: usize,
    pub screen_height: usize,
}
