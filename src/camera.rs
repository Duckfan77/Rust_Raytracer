use crate::vec3::*;
use crate::ray::Ray;
use crate::util::*;
use std::f64;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: &Vec3,
        lookat: &Vec3,
        vup: &Vec3,
        vfov: f64,  // vertical fov in degrees
        aspect_ratio: f64
    ) -> Camera {
        let theta = degs_to_rads(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0*h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(*lookfrom - *lookat);
        let u = unit_vector(cross(*vup, w));
        let v = cross(w, u);

        let origin = lookfrom;
        let horizontal = viewport_width*u;
        let vertical = viewport_height * v;
        let lower_left_corner = *origin - horizontal/2.0 - vertical/2.0 - w;

        Camera {origin: *origin,
                horizontal: horizontal,
                vertical: vertical,
                lower_left_corner: lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(&self.origin, &(self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin))
    }
}