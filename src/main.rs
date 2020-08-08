use std::io::{Write, stderr};

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
            let r: f64 = i / (image_width-1) as f64;
            let g: f64 = j / (image_height-1) as f64;
            let b: f64 = 0.25;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
