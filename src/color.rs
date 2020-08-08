use crate::vec3;
use std::io;
use std::fmt;

#[allow(dead_code)]
pub fn write_color(mut out: impl io::Write, pixel: vec3::Color) -> ()
{
    let _ = write!(out, "{} {} {}\n", (255.999 * pixel.x()) as u32, (255.999 * pixel.y()) as u32, (255.999 * pixel.z()) as u32);
}

#[allow(dead_code)]
pub fn write_color_str<T: fmt::Write>(mut out: T, pixel: vec3::Color) -> T{
    write!(out, "{} {} {}\n", (255.999 * pixel.x()) as u32, (255.999 * pixel.y()) as u32, (255.999 * pixel.z()) as u32).expect("Error Writing to out");
    return out;
}