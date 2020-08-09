use crate::vec3::*;
use crate::ray::Ray;
use crate::util::*;
use std::f64;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: &Vec3,
        lookat: &Vec3,
        vup: &Vec3,
        vfov: f64,  // vertical fov in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degs_to_rads(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0*h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(*lookfrom - *lookat);
        let u = unit_vector(cross(*vup, w));
        let v = cross(w, u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width*u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = *origin - horizontal/2.0 - vertical/2.0 - focus_dist * w;

        Camera {origin: *origin,
                horizontal: horizontal,
                vertical: vertical,
                lower_left_corner: lower_left_corner,
                u: u, v: v, w: w,
                lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v*rd.y();

        Ray::new(&(self.origin + offset), &(self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset))
    }
}