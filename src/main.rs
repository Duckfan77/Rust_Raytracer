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
mod moving_sphere;
mod aabb;
mod bvh;
mod texture;
mod perlin;
mod aarect;
mod boxes;

use vec3::*;
use util::*;
use materials::*;
use texture::*;
use hittable::*;

fn random_scene() -> hittable_list::HittableList {
    let mut world = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let checker: Rc<dyn Texture> = Rc::new(CheckerTexture::new_clr(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let mat_gnd: Rc<dyn Material> = Rc::new(Lambertian::new_txtr(&checker));
    world.add(Rc::new(sphere::Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, Rc::clone(&mat_gnd))));

    let mut spheres = hittable_list::HittableList {objects: Vec::with_capacity(484)};

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
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    spheres.add(Rc::new(moving_sphere::MovingSphere::new(center, center2, 0.0, 1.0, 0.2, Rc::clone(&mat_sphere))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    mat_sphere = Rc::new(Metal::new(albedo, fuzz));
                    spheres.add(Rc::new(sphere::Sphere::new(center, 0.2, Rc::clone(&mat_sphere))));
                } else {
                    // glass
                    mat_sphere = Rc::new(Dialectric::new(1.5));
                    spheres.add(Rc::new(sphere::Sphere::new(center, 0.2, Rc::clone(&mat_sphere))));
                }

            }
        }
    }

    let mat1: Rc<dyn Material> = Rc::new(Dialectric::new(1.5));
    world.add(Rc::new(sphere::Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, Rc::clone(&mat1))));

    let mat2: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(sphere::Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, Rc::clone(&mat2))));

    let mat3: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(sphere::Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, Rc::clone(&mat3))));

    //let mut out = hittable_list::HittableList {objects: Vec::with_capacity(10)};
    world.add(Rc::new(bvh::BvhNode::new_l(&mut spheres, 0.0, 1.0)));

    return world
}

fn two_spheres() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let checker: Rc<dyn Texture> = Rc::new(CheckerTexture::new_clr(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

    objects.add(Rc::new(sphere::Sphere::new(Point::new(0.0, -10.0, 0.0), 10.0, Rc::new(Lambertian::new_txtr(&checker)))));
    objects.add(Rc::new(sphere::Sphere::new(Point::new(0.0,  10.0, 0.0), 10.0, Rc::new(Lambertian::new_txtr(&checker)))));

    return objects
}

fn two_perlin_spheres() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let pertext: Rc<dyn Texture> = Rc::new(MarbleNoiseTexture::new_sc(4.0));
    let pertext2: Rc<dyn Texture> = Rc::new(NoiseTexture::new_sc(5.0));

    objects.add(Rc::new(sphere::Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new_txtr(&pertext2)))));
    objects.add(Rc::new(sphere::Sphere::new(Point::new(0.0,  2.0, 0.0), 2.0, Rc::new(Lambertian::new_txtr(&pertext)))));

    return objects
}

fn earth() -> hittable_list::HittableList {
    let earth_txtr: Rc<dyn Texture> = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface: Rc<dyn Material> = Rc::new(Lambertian::new_txtr(&earth_txtr));
    let globe = Rc::new(sphere::Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, Rc::clone(&earth_surface)));

    return hittable_list::HittableList::new(globe)
}

fn simple_light() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let pertext: Rc<dyn Texture> = Rc::new(MarbleNoiseTexture::new_sc(4.0));
    objects.add(Rc::new(sphere::Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new_txtr(&pertext)))));
    objects.add(Rc::new(sphere::Sphere::new(Point::new(0.0,  2.0, 0.0), 2.0, Rc::new(Lambertian::new_txtr(&pertext)))));

    let difflight: Rc<dyn Material> = Rc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    //let grnlight: Rc<dyn Material> = Rc::new(DiffuseLight::new(Color::new(0.0, 12.0, 0.0)));
    objects.add(Rc::new(aarect::XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, Rc::clone(&difflight))));
    objects.add(Rc::new(sphere::Sphere::new(Point::new(0.0, 7.0, 0.0), 2.0, Rc::clone(&difflight))));

    return objects
}

fn cornell_box() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let red:   Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Rc<dyn Material> = Rc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    objects.add(Rc::new(aarect::YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Rc::clone(&green))));
    objects.add(Rc::new(aarect::YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Rc::clone(&red))));
    objects.add(Rc::new(aarect::XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, Rc::clone(&light))));
    objects.add(Rc::new(aarect::XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Rc::clone(&white))));
    objects.add(Rc::new(aarect::XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Rc::clone(&white))));
    objects.add(Rc::new(aarect::XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Rc::clone(&white))));

    let mut box1: Rc<dyn Hittable> = Rc::new(boxes::Box::new(&Point::new(0.0, 0.0, 0.0), &Point::new(165.0, 330.0, 165.0), Rc::clone(&white)));
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let mut box2: Rc<dyn Hittable> = Rc::new(boxes::Box::new(&Point::new(0.0, 0.0, 0.0), &Point::new(165.0, 165.0, 165.0), Rc::clone(&white)));
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);

    return objects
}

fn ray_color(r: &ray::Ray, background: &Color, world: &dyn hittable::Hittable, depth: u32) -> Color{
    let mut rec = hittable::HitRecord::new();

    // Just return no light if past ray bounce limit
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0)
    }

    // If the ray hits nothing, return the background
    if !world.hit(r, 0.0001, INFINITY, &mut rec) {
        return *background
    }

    let (success, scattered, attenuation) = rec.mat_ptr.as_ref().scatter(r, &rec);
    let emitted = rec.mat_ptr.as_ref().emitted(rec.u, rec.v, &rec.p);

    if !success {
        return emitted
    }

    return emitted + attenuation * ray_color(&scattered, background, world, depth-1)
}

fn main() {
    //Image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 400;
    let mut sample_per_pixel = 100;
    let max_depth = 50;

    // World
    let world: hittable_list::HittableList;

    let lookfrom: Point;
    let lookat: Point;
    #[allow(unused_assignments)]
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    #[allow(unused_assignments)]
    let mut background = Color::new(0.0, 0.0, 0.0);

    match 0 {
        1 => {
            world = random_scene();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point::new(13.0, 2.0, 3.0);
            lookat = Point::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }

        2 => {
            world = two_spheres();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point::new(13.0, 2.0, 3.0);
            lookat = Point::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }

        3 => {
            world = two_perlin_spheres();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point::new(13.0, 2.0, 3.0);
            lookat = Point::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }

        4 => {
            world = earth();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point::new(13.0, 2.0, 3.0);
            lookat = Point::new(0.0, 0.0, 0.0);
            vfov = 20.0
        }

        5 => {
            world = simple_light();
            sample_per_pixel = 400;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point::new(26.0, 3.0, 6.0);
            lookat = Point::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }

        6 | _ => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            sample_per_pixel = 200;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point::new(278.0, 278.0, -800.0);
            lookat = Point::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }

    // Camera
    let vup = Point::new(0.0, 1.0, 0.0);
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let dist_to_focus = 10.0;

    let cam = camera::Camera::new(&lookfrom, &lookat, &vup, vfov, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

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

                pixel += ray_color(&r, &background, &world, max_depth);
            }

            color::write_color(&mut outlock, pixel, sample_per_pixel);
        }
    }
}
