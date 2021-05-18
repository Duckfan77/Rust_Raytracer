#![allow(dead_code)]

use std::io::{Write, stderr};
use std::sync::Arc;
use std::f64;
use rayon::prelude::*;

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
mod constant_medium;
mod picture;

use vec3::*;
use util::*;
use materials::*;
use texture::*;
use hittable::*;
use picture::Picture;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn random_scene() -> hittable_list::HittableList {
    let mut world = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let checker: Arc<dyn Texture + Sync + Send> = Arc::new(CheckerTexture::new_clr(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let mat_gnd: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::new_txtr(&checker));
    world.add(Arc::new(sphere::Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&mat_gnd))));

    let mut spheres = hittable_list::HittableList {objects: Vec::with_capacity(484)};

    for a in -11..11 {
        let a = a as f64;
        for b in -11..11 {
            let b = b as f64;

            let choose_mat = random_double();
            let center = Point::new(a + 0.9*random_double(), 0.2, b + 0.9*random_double());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat_sphere: Arc<dyn Material + Sync + Send>;

                if choose_mat < 0.5 {
                    // difuse
                    let albedo = random() * random();
                    mat_sphere = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    if choose_mat < 0.25 {
                        spheres.add(Arc::new(moving_sphere::MovingSphere::new(center, center2, 0.0, 1.0, 0.2, mat_sphere)));
                    } else {
                        spheres.add(Arc::new(sphere::Sphere::new(center, 0.2, mat_sphere)))
                    }
                } else if choose_mat < 0.75 {
                    // metal
                    let albedo = random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    mat_sphere = Arc::new(Metal::new(albedo, fuzz));
                    spheres.add(Arc::new(sphere::Sphere::new(center, 0.2, Arc::clone(&mat_sphere))));
                } else {
                    // glass
                    mat_sphere = Arc::new(Dialectric::new(1.5));
                    spheres.add(Arc::new(sphere::Sphere::new(center, 0.2, Arc::clone(&mat_sphere))));
                }

            }
        }
    }

    let mat1: Arc<dyn Material  + Sync + Send> = Arc::new(Dialectric::new(1.5));
    world.add(Arc::new(sphere::Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, Arc::clone(&mat1))));

    let mat2: Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(sphere::Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, Arc::clone(&mat2))));

    let mat3: Arc<dyn Material  + Sync + Send> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(sphere::Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, Arc::clone(&mat3))));

    //let mut out = hittable_list::HittableList {objects: Vec::with_capacity(10)};
    world.add(Arc::new(bvh::BvhNode::new_l(&mut spheres, 0.0, 1.0)));

    return world
}

fn two_spheres() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let checker: Arc<dyn Texture + Sync + Send> = Arc::new(CheckerTexture::new_clr(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, -10.0, 0.0), 10.0, Arc::new(Lambertian::new_txtr(&checker)))));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0,  10.0, 0.0), 10.0, Arc::new(Lambertian::new_txtr(&checker)))));

    return objects
}

fn two_perlin_spheres() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(MarbleNoiseTexture::new_sc(4.0));
    let pertext2: Arc<dyn Texture + Sync + Send> = Arc::new(NoiseTexture::new_sc(5.0));

    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new_txtr(&pertext2)))));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0,  2.0, 0.0), 2.0, Arc::new(Lambertian::new_txtr(&pertext)))));

    return objects
}

fn earth() -> hittable_list::HittableList {
    let earth_txtr: Arc<dyn Texture + Sync + Send> = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface: Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new_txtr(&earth_txtr));
    let globe = Arc::new(sphere::Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, Arc::clone(&earth_surface)));

    return hittable_list::HittableList::new(globe)
}

fn simple_light() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(TurbNoiseTexture::new_sc(0.5));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new_txtr(&pertext)))));
    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(MarbleNoiseTexture::new_sc_clr(3.5, Color::new(1.0, 1.0, 1.0)));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0,  2.0, 0.0), 2.0, Arc::new(Lambertian::new_txtr(&pertext)))));

    let difflight: Arc<dyn Material  + Sync + Send> = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    //let grnlight: Arc<dyn Material  + Sync + Send> = Arc::new(DiffuseLight::new(Color::new(0.0, 12.0, 0.0)));
    objects.add(Arc::new(aarect::XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, Arc::clone(&difflight))));
    //objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, 7.0, 0.0), 2.0, Arc::clone(&difflight))));

    return objects
}

fn cornell_box() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let red:   Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material  + Sync + Send> = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(aarect::YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&green))));
    objects.add(Arc::new(aarect::YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&red))));
    objects.add(Arc::new(aarect::XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, Arc::clone(&light))));
    objects.add(Arc::new(aarect::XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&white))));
    objects.add(Arc::new(aarect::XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white))));
    objects.add(Arc::new(aarect::XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white))));

    let mut box1: Arc<dyn Hittable + Sync + Send> = Arc::new(boxes::Box::new(&Point::new(0.0, 0.0, 0.0), &Point::new(165.0, 330.0, 165.0), Arc::clone(&white)));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let mut box2: Arc<dyn Hittable + Sync + Send> = Arc::new(boxes::Box::new(&Point::new(0.0, 0.0, 0.0), &Point::new(165.0, 165.0, 165.0), Arc::clone(&white)));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);

    return objects
}

fn cornell_smoke() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let red:   Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material  + Sync + Send> = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    objects.add(Arc::new(aarect::YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&green))));
    objects.add(Arc::new(aarect::YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&red))));
    objects.add(Arc::new(aarect::XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, Arc::clone(&light))));
    objects.add(Arc::new(aarect::XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white))));
    objects.add(Arc::new(aarect::XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&white))));
    objects.add(Arc::new(aarect::XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white))));

    let mut box1: Arc<dyn Hittable + Sync + Send> = Arc::new(boxes::Box::new(&Point::new(0.0, 0.0, 0.0), &Point::new(165.0, 330.0, 165.0), Arc::clone(&white)));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable + Sync + Send> = Arc::new(boxes::Box::new(&Point::new(0.0, 0.0, 0.0), &Point::new(165.0, 165.0, 165.0), Arc::clone(&white)));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));

    objects.add(Arc::new(constant_medium::ConstantMedium::new(box1, 0.01, Color::new(0.0, 0.0, 0.0))));
    objects.add(Arc::new(constant_medium::ConstantMedium::new(box2, 0.01, Color::new(1.0, 1.0, 1.0))));

    return objects
}

fn final_scene() -> hittable_list::HittableList {
    let mut boxes1 = hittable_list::HittableList {objects: Vec::with_capacity(10)};
    let ground: Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64*w;
            let z0 = -1000.0 + j as f64*w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(boxes::Box::new(&Point::new(x0, y0, z0), &Point::new(x1, y1, z1), Arc::clone(&ground))));
        }
    }

    let mut objects = hittable_list::HittableList::new(Arc::new(bvh::BvhNode::new_l(&mut boxes1, 0.0, 1.0)));

    let light: Arc<dyn Material  + Sync + Send> = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(aarect::XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, Arc::clone(&light))));

    let center1 = Point::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let mov_sph_mat: Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(moving_sphere::MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, Arc::clone(&mov_sph_mat))));

    objects.add(Arc::new(sphere::Sphere::new(Point::new(260.0, 150.0, 45.0), 50.0, Arc::new(Dialectric::new(1.5)))));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, 150.0, 145.0), 50.0, Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 10.0))
    )));

    let mut boundary: Arc<dyn Hittable + Sync + Send> = Arc::new(sphere::Sphere::new(Point::new(360.0, 150.0, 145.0), 70.0, Arc::new(Dialectric::new(1.5))));
    objects.add(Arc::clone(&boundary));
    objects.add(Arc::new(constant_medium::ConstantMedium::new(Arc::clone(&boundary), 0.2, Color::new(0.2, 0.4, 0.9))));
    boundary = Arc::new(sphere::Sphere::new(Point::new(0.0, 0.0, 0.0), 5000.0, Arc::new(Dialectric::new(1.5))));
    objects.add(Arc::new(constant_medium::ConstantMedium::new(Arc::clone(&boundary), 0.0001, Color::new(1.0, 1.0, 1.0))));

    let emat: Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new_txtr(&(Arc::new(ImageTexture::new("earthmap.jpg")) as Arc<dyn Texture + Sync + Send>)));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(400.0, 200.0, 400.0), 100.0, emat)));
    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(MarbleNoiseTexture::new_sc(0.1));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(220.0, 280.0, 300.0), 80.0, Arc::new(Lambertian::new_txtr(&pertext)))));

    let mut boxes2 = hittable_list::HittableList {objects: Vec::with_capacity(10)};
    let white: Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(sphere::Sphere::new(random_range(0.0, 165.0), 10.0, Arc::clone(&white))));
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(bvh::BvhNode::new_l(&mut boxes2, 0.0, 1.0)), 15.0)),
        &Vec3::new(-100.0, 270.0, 395.0)
    )));

    return objects
}

fn glow_earth() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::new()};

    let earth_txtr: Arc<dyn Texture + Sync + Send> = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface: Arc<dyn Material  + Sync + Send> = Arc::new(DiffuseLight::new_txtr(earth_txtr));
    let globe = Arc::new(sphere::Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, Arc::clone(&earth_surface)));
    objects.add(globe);

    let _wall: Arc<dyn Material  + Sync + Send> = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0)));
    //objects.add(Arc::new(aarect::YZRect))

    return objects;
}

fn bg() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(TurbNoiseTexture::new_sc(0.5));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new_txtr(&pertext)))));
    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(MarbleNoiseTexture::new_sc_clr(3.5, Color::new(1.0/3.0, 2.0/3.0, 2.0/3.0)*1.5));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, 3.0, 0.0), 2.0, Arc::new(DiffuseLight::new_txtr(pertext)))));

    //let difflight: Arc<dyn Material  + Sync + Send> = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    //let grnlight: Arc<dyn Material  + Sync + Send> = Arc::new(DiffuseLight::new(Color::new(0.0, 12.0, 0.0)));
    //objects.add(Arc::new(aarect::XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, Arc::clone(&difflight))));
    //objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, 7.0, 0.0), 2.0, Arc::clone(&difflight))));

    let wall: Arc<dyn Material  + Sync + Send> = Arc::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.0));
    objects.add(Arc::new(aarect::YZRect::new(0.0, 6.0, -3.0, 3.0, -4.0, wall)));

    return objects
}

fn pandorba() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(TurbNoiseTexture::new_sc_clr(0.5, Color::new(0.6078, 0.4627, 0.3255)));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new_txtr(&pertext)))));
    //let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(DualMarbleNoiseTexture::new_sc_clr_weight(1.0, Color::new(0.0, 0.0, 1.0)*1.0, Color::new(1.0, 1.0, 1.0)*1.0, 1.5));
    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(FragmentNoiseTexture::new(3, Color::new(0.0, 0.6, 0.8)*1.0, Color::new(0.8, 0.8, 1.0)*1.0, -0.1, 0.05, 0.1, 1.0, 1.0, 2.0));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, 3.0, 0.0), 2.0, Arc::new(DiffuseLight::new_txtr(pertext)))));

    //let _fog: Arc<dyn Hittable + Sync + Send> = Arc::new(sphere::Sphere::new(Point::new(0.0, 3.0, 0.0), 10.0, Arc::new(Lambertian::new(Color::new_e()))));
    //objects.add(Arc::new(constant_medium::ConstantMedium::new(_fog, 0.1, Color::new(1.0, 1.0, 1.0))));

    return objects
}

fn noises() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {objects: Vec::with_capacity(10)};

    let perlin: Arc<dyn Texture + Sync + Send> = Arc::new(NoiseTexture::new_sc(10.0));
    let turb: Arc<dyn Texture + Sync + Send> = Arc::new(TurbNoiseTexture::new_sc(10.0));
    let marble: Arc<dyn Texture + Sync + Send> = Arc::new(MarbleNoiseTexture::new_sc(10.0));

    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, -2.0, 0.0), 1.0, Arc::new(Lambertian::new_txtr(&perlin)))));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0,  0.0, 0.0), 1.0, Arc::new(Lambertian::new_txtr(&turb)))));
    objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0,  2.0, 0.0), 1.0, Arc::new(Lambertian::new_txtr(&marble)))));

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
    //Get Initial Values from Command Line
    let matches = App::new("Rust Raytracer")
        .arg(Arg::with_name("Out File")
            .value_name("FILE")
            .index(1)
            .required(true))
        .arg(Arg::with_name("Out Type")
            .value_name("TYPE")
            .possible_values(&picture::PictureType::variants())
            .case_insensitive(true)
            .required(true)
            .index(2))
        .arg(Arg::with_name("Scene Number")
            .value_name("SCENE")
            .long("scene")
            .short("s")
            .default_value("0")
            .validator(|x| match x.parse::<u32>(){
                Ok(y) => {if y > MAX_SCENE {Err("The value is above the top scene number".to_string())} else {Ok(())}},
                Err(_) => Err("The value is not a valid unsigned integer".to_string())
            }))
        .arg(Arg::with_name("Image Width")
            .value_name("WIDTH")
            .long("width")
            .short("w")
            .validator(|x| match x.parse::<u32>(){
                Ok(y) => {if y == 0 {Err(String::from("The value must be non-zero"))} else {Ok(())}},
                Err(_) => Err(String::from("The value is not a valid unsigned integer")),
            }))
        .arg(Arg::with_name("Sample Count")
            .value_name("SAMPLES")
            .long("samples")
            .short("c")
            .validator(|x| match x.parse::<u32>(){
                Ok(y) => {if y == 0 {Err(String::from("The value must be non-zero"))} else {Ok(())}},
                Err(_) => Err(String::from("The value is not a valid unsigned integer")),
            }))
        .arg(Arg::with_name("Bounce Depth")
            .value_name("DEPTH")
            .long("depth")
            .short("d")
            .validator(|x| match x.parse::<u32>(){
                Ok(y) => {if y== 0 {Err(String::from("The value must be non-zero"))} else {Ok(())}},
                Err(_) => Err(String::from("The value is not a valid unsigned integer")),
            }))
        .get_matches();

    let scene = value_t!(matches, "Scene Number", u32).unwrap();
    let outname = matches.value_of("Out File").unwrap();
    let outtype = value_t!(matches, "Out Type", picture::PictureType).unwrap();

    //Picture defaults - may be overridden by scene or by user
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 1920;
    let mut sample_per_pixel = 100;
    let mut max_depth = 50;

    // World
    let world: hittable_list::HittableList;

    //Camera Defaults, may be overridden by specific scene
    let lookfrom: Point;
    let lookat: Point;
    #[allow(unused_assignments)]
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    #[allow(unused_assignments)]
    let mut background = Color::new(0.0, 0.0, 0.0);

    match scene {
        1 => {
            world = random_scene();
            image_width = 1200;
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

        6 => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            sample_per_pixel = 200;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point::new(278.0, 278.0, -800.0);
            lookat = Point::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }

        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            sample_per_pixel = 200;
            lookfrom = Point::new(278.0, 278.0, -800.0);
            lookat = Point::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }

        8 => {
            world = final_scene();
            aspect_ratio = 1.0;
            image_width = 800;
            sample_per_pixel = 10000;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point::new(478.0, 278.0, -600.0);
            lookat = Point::new(278.0, 278.0, 0.0);
            vfov = 400.0;
        }

        9 => {
            world = earth();
            background = Color::new(0.0, 0.0, 0.00);
            lookfrom = Point::new(13.0, 2.0, 3.0);
            lookat = Point::new(0.0, 0.0, 0.0);
            vfov = 20.0
        }

        10 => {
            world = bg();
            sample_per_pixel = 400;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point::new(26.0, 3.0, 6.0);
            lookat = Point::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }

        11 => {
            world = pandorba();
            sample_per_pixel = 400;
            background = Color::new(1.0, 1.0, 1.0)*0.01;
            lookfrom = Point::new(26.0, 3.0, 6.0);
            lookat = Point::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }

        12 | _=> {
            world = noises();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point::new(13.0, 2.0, 3.0);
            lookat = Point::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
    }

    //Collect User Values and Override Scene if user provided
    match value_t!(matches, "Image Width", u32){
        Ok(y) => {image_width = y}
        Err(_) => {}
    }

    match value_t!(matches, "Samplke Count", u32){
        Ok(y) => {sample_per_pixel = y}
        Err(_) => {}
    }

    match value_t!(matches, "Bounce Depth", u32){
        Ok(y) => {max_depth = y}
        Err(_) => {}
    }

    println!("{} {} {}", image_width, sample_per_pixel, max_depth);

    // Camera
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = camera::Camera::new(&lookfrom, &lookat, &vup, vfov, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

    //Image
    let mut img = Picture::new(image_width, aspect_ratio, sample_per_pixel, &String::from(outname), outtype).expect("Error making image");

    //Render
    let err = stderr();
    let mut errlock = err.lock();

    let mut row: Vec<Color> = Vec::new();

    for j in (0..img.height()).rev() {
        let r = img.height()-j-1;
        write!(errlock, "\rScanlines remaining: {} ", j).expect("Fail to write to Err");
        errlock.flush().expect("Fail to flush stderr");

        (0..img.width()).into_par_iter().map(|i| {
            (0..img.samples_per_pixel()).into_iter().map(|_| {
                let u = (i as f64 + random_double())/(img.width() - 1) as f64;
                let v = (j as f64 + random_double())/(img.height() - 1) as f64;

                let r = cam.get_ray(u, v);

                ray_color(&r, &background, &world, max_depth)
            }).fold( Color::new_e(), |acc, x| acc + x)
        }).collect_into_vec(&mut row);

        img.write_row(&row, r).expect("Error Writing Row");
    }

    img.save().expect("Error Saving To File");
}
