#![allow(dead_code)]

use std::io::{Write, stderr, stdout};
use std::f64;
mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;

use vec3::*;

fn hit_sphere(center: &Point, radius: f64, r: &ray::Ray) -> f64{
    let oc: Vec3 = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = dot(oc, r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;

    if discriminant < 0.0 {
        return -1.0
    }else{
        return (-half_b - f64::sqrt(discriminant)) / a
    }
}

fn ray_color(r: &ray::Ray) -> Color{
    let t = hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, r);
    if t>0.0 {
        //assume Normals are unit vectors, not enforced
        let N = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(N.x()+1.0, N.y()+1.0, N.z()+1.0);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    return (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0);
}

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..=(image_height-1)).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().expect("Fail to flush stderr");

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = ray::Ray::new(&origin, &(lower_left_corner + u*horizontal + v*vertical - origin));
            let pixel = ray_color(&r);
            color::write_color(stdout(), pixel);
        }
    }
}
