#![allow(dead_code)]

use std::io::{Write, stderr, stdout};
use std::rc::Rc;
use std::f64;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod util;
mod camera;

use vec3::*;
use util::*;

fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable) -> Color{
    let mut rec = hittable::HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
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
    let sample_per_pixel = 100;

    // World
    let mut world = hittable_list::HittableList {objects: Vec::with_capacity(10)};
    world.add(Rc::new(sphere::Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(sphere::Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = camera::Camera::new();

    //Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..=(image_height-1)).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().expect("Fail to flush stderr");

        for i in 0..image_width {
            let mut pixel = Color::new(0.0, 0.0, 0.0);

            for _ in 0..sample_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);

                pixel += ray_color(&r, &world);
            }

            color::write_color(stdout(), pixel, sample_per_pixel);
        }
    }
}
