#![allow(dead_code)]

use std::io::{Write, stderr, stdout};
use std::rc::*;
use std::f64;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod util;
mod camera;
mod materials;

use vec3::*;
use util::*;
use materials::*;

fn random_scene() -> hittable_list::HittableList {
    let mut world = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let mat_gnd: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(sphere::Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, Rc::clone(&mat_gnd))));

    for a in -11..11 {
        let a = a as f64;
        for b in -11..11 {
            let b = b as f64;

            let choose_mat = random_double();
            let center = Point::new(a + 0.9*random_double(), 0.2, b + 0.9*random_double());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat_sphere: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // difuse
                    let albedo = random() * random();
                    mat_sphere = Rc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    mat_sphere = Rc::new(Metal::new(albedo, fuzz));
                } else {
                    // glass
                    mat_sphere = Rc::new(Dialectric::new(1.5));
                }

                world.add(Rc::new(sphere::Sphere::new(center, 0.2, Rc::clone(&mat_sphere))));
            }
        }
    }

    let mat1: Rc<dyn Material> = Rc::new(Dialectric::new(1.5));
    world.add(Rc::new(sphere::Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, Rc::clone(&mat1))));

    let mat2: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(sphere::Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, Rc::clone(&mat2))));

    let mat3: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(sphere::Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, Rc::clone(&mat3))));

    return world
}

fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable, depth: u32) -> Color{
    let mut rec = hittable::HitRecord::new();

    // Just return no light if past ray bounce limit
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0)
    }

    if world.hit(r, 0.0001, INFINITY, &mut rec) {
        let (success, scattered, attenuation) = rec.mat_ptr.as_ref().scatter(r, &rec);

        if success {
            return attenuation * ray_color(&scattered, world, depth-1)
        }
        return Color::new(0.0, 0.0, 0.0)
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    return (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0);
}

fn main() {
    //Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let sample_per_pixel = 100;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = camera::Camera::new(&lookfrom, &lookat, &vup, 20.0, aspect_ratio, aperture, dist_to_focus);

    //Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    let out = stdout();
    let mut outlock = out.lock();

    let err = stderr();
    let mut errlock = err.lock();

    for j in (0..=(image_height-1)).rev() {
        write!(errlock, "\rScanlines remaining: {} ", j).expect("Fail to write to Err");
        errlock.flush().expect("Fail to flush stderr");

        for i in 0..image_width {
            let mut pixel = Color::new(0.0, 0.0, 0.0);

            for _ in 0..sample_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);

                pixel += ray_color(&r, &world, max_depth);
            }

            color::write_color(&mut outlock, pixel, sample_per_pixel);
        }
    }
}
