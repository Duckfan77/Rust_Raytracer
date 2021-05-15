use crate::vec3;
use std::io;
use std::fmt;
use crate::util::*;

#[allow(dead_code)]
pub fn write_color_ppm(out: &mut impl io::Write, pixel: vec3::Color, sample_count: u32) -> ()
{
    let mut r = pixel.x();
    let mut g = pixel.y();
    let mut b = pixel.z();

    let scale = 1.0 / sample_count as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    let _ = write!(out, "{} {} {}\n", (256.0 * clamp(r, 0.0, 0.999)) as u32, (256.0 * clamp(g, 0.0, 0.999)) as u32, (256.0 * clamp(b, 0.0, 0.999)) as u32);
}

#[allow(dead_code)]
pub fn write_pixel_str_ppm<T: fmt::Write>(out: &mut T, pixel: vec3::Color, sample_count: u32) {//-> T{
    let mut r = pixel.x();
    let mut g = pixel.y();
    let mut b = pixel.y();

    let scale = 1.0 / sample_count as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    let _ = write!(out, "{} {} {}\n", (256.0 * clamp(r, 0.0, 0.999)) as u32, (256.0 * clamp(g, 0.0, 0.999)) as u32, (256.0 * clamp(b, 0.0, 0.999)) as u32);
}