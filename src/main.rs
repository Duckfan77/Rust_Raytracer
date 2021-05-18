#![allow(dead_code)]

use std::io::{Write, stderr};
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
mod scene;

use vec3::*;
use util::*;
use picture::Picture;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

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
    //Picture and Camera defaults - may be overridden by scene or by user
    let mut scene_dat = scene::SceneData{
        image_width: 1920,
        background: Color::new(0.0, 0.0, 0.0),
        lookfrom: Point::new_e(),
        lookat: Point::new_e(),
        vfov: 40.0,
        aperture: 0.0,
        sample_per_pixel: 100,
        aspect_ratio: 16.0 / 9.0,
    };
    let mut max_depth = 50;

    let iw_str = format!("{}",scene_dat.image_width);
    let sc_str = format!("{}",scene_dat.sample_per_pixel);
    let md_str = format!("{}",max_depth);

    //Get Initial Values from Command Line
    let matches = App::new("Rust Raytracer")
        .arg(Arg::with_name("Out File")
            .value_name("FILE")
            .index(1)
            .help("Sets the output file to write to")
            .required(true))
        .arg(Arg::with_name("Out Type")
            .value_name("TYPE")
            .possible_values(&picture::PictureType::variants())
            .case_insensitive(true)
            .required(true)
            .help("Sets the image type.\n\nFor ppm image type, FILE exension does not matter. \n\n\
            For rgb image types, FILE must be PNG or JPG, which will define output image type\n\
            rgb8 and rgb16 set the number of bits per channel when preparing image to save\n")
            .index(2))
        .arg(Arg::with_name("Scene Number")
            .value_name("SCENE")
            .long("scene")
            .short("s")
            .possible_values(&scene::Scene::variants())
            .default_value("CornellBox")
            .case_insensitive(true)
            .help("Scene to Display"))
        .arg(Arg::with_name("Image Width")
            .value_name("WIDTH")
            .long("width")
            .short("w")
            .help("Width of the output image in pixels")
            .default_value(&iw_str)
            .validator(|x| match x.parse::<u32>(){
                Ok(y) => {if y == 0 {Err(String::from("The value must be non-zero"))} else {Ok(())}},
                Err(_) => Err(String::from("The value is not a valid unsigned integer")),
            }))
        .arg(Arg::with_name("Sample Count")
            .value_name("SAMPLES")
            .long("samples")
            .short("c")
            .help("Samples per pixel. More generally produces a sharper image, but greatly increases render time")
            .default_value(&sc_str)
            .validator(|x| match x.parse::<u32>(){
                Ok(y) => {if y == 0 {Err(String::from("The value must be non-zero"))} else {Ok(())}},
                Err(_) => Err(String::from("The value is not a valid unsigned integer")),
            }))
        .arg(Arg::with_name("Bounce Depth")
            .value_name("DEPTH")
            .long("depth")
            .short("d")
            .help("Maximum number of bounces permitted per ray. Diminishing returns for quality when increased, unlikely to need changing from default")
            .default_value(&md_str)
            .validator(|x| match x.parse::<u32>(){
                Ok(y) => {if y== 0 {Err(String::from("The value must be non-zero"))} else {Ok(())}},
                Err(_) => Err(String::from("The value is not a valid unsigned integer")),
            }))
        .get_matches();

    let scene = value_t!(matches, "Scene Number", scene::Scene).unwrap();
    let outname = matches.value_of("Out File").unwrap();
    let outtype = value_t!(matches, "Out Type", picture::PictureType).unwrap();

    // World
    let world = scene::match_scene(scene, &mut scene_dat);

    //Collect User Values and Override Scene if user provided
    match value_t!(matches, "Image Width", u32){
        Ok(y) => {if matches.occurrences_of("Image Width") != 0 {scene_dat.image_width = y}}
        Err(_) => {}
    }

    match value_t!(matches, "Sample Count", u32){
        Ok(y) => {if matches.occurrences_of("Sample Count") != 0 {scene_dat.sample_per_pixel = y}}
        Err(_) => {}
    }

    match value_t!(matches, "Bounce Depth", u32){
        Ok(y) => {max_depth = y}
        Err(_) => {}
    }

    // Camera
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = camera::Camera::new(&scene_dat.lookfrom, &scene_dat.lookat, &vup, scene_dat.vfov, scene_dat.aspect_ratio, scene_dat.aperture, dist_to_focus, 0.0, 1.0);

    //Image
    let mut img = Picture::new(scene_dat.image_width, scene_dat.aspect_ratio, scene_dat.sample_per_pixel, &String::from(outname), outtype).expect("Error making image");

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

                ray_color(&r, &scene_dat.background, &world, max_depth)
            }).fold( Color::new_e(), |acc, x| acc + x)
        }).collect_into_vec(&mut row);

        img.write_row(&row, r).expect("Error Writing Row");
    }

    img.save().expect("Error Saving To File");
}
