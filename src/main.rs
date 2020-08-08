#![allow(dead_code)]

use std::io::{Write, stderr, stdout};
mod vec3;
mod color;
mod ray;

fn main() {
    //Image
    let image_width = 256;
    let image_height = 256;

    //Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..=(image_height-1)).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().expect("Fail to flush stderr");

        let j = j as f64;
        for i in 0..image_width {
            let i = i as f64;

            let pixel = vec3::Color::new(i / (image_width-1) as f64, j / (image_height - 1) as f64, 0.25);
            color::write_color(stdout(), pixel);
        }
    }
}
