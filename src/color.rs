use crate::vec3;
use std::io;
use std::fmt;
use crate::util::*;
use image::{Rgb, ImageBuffer, RgbImage};

fn convert_pixel(pixel: vec3::Color, sample_count: u32) -> (f64, f64, f64){
    let mut r = pixel.x();
    let mut g = pixel.y();
    let mut b = pixel.z();

    let scale = 1.0 / sample_count as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    return (r,g,b)
}

#[allow(dead_code)]
pub fn write_color_ppm(out: &mut impl io::Write, pixel: vec3::Color, sample_count: u32) -> ()
{
    let (r,g,b) = convert_pixel(pixel, sample_count);

    let _ = write!(out, "{} {} {}\n", (256.0 * clamp(r, 0.0, 0.999)) as u32, (256.0 * clamp(g, 0.0, 0.999)) as u32, (256.0 * clamp(b, 0.0, 0.999)) as u32);
}

#[allow(dead_code)]
pub fn write_pixel_str_ppm<T: fmt::Write>(out: &mut T, pixel: vec3::Color, sample_count: u32) {//-> T{
    let (r,g,b) = convert_pixel(pixel, sample_count);

    let _ = write!(out, "{} {} {}\n", (256.0 * clamp(r, 0.0, 0.999)) as u32, (256.0 * clamp(g, 0.0, 0.999)) as u32, (256.0 * clamp(b, 0.0, 0.999)) as u32);
}
pub fn write_pixel_img_8bpp(x: u32, y: u32, pixel: vec3::Color, sample_count: u32, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let (r,g,b) = convert_pixel(pixel, sample_count);

    let max_color = std::u8::MAX as f64 + 1.0;

    img.put_pixel(x, y, Rgb(
        [(max_color * clamp(r, 0.0, 0.999)) as u8,
         (max_color * clamp(g, 0.0, 0.999)) as u8,
         (max_color * clamp(b, 0.0, 0.999)) as u8]
    ))
}

pub fn write_pixel_img_16bpp(x: u32, y: u32, pixel: vec3::Color, sample_count: u32, img: &mut ImageBuffer<Rgb<u16>, Vec<u16>>) {
    let (r,g,b) = convert_pixel(pixel, sample_count);

    let max_color = std::u16::MAX as f64 + 1.0;

    img.put_pixel(x, y, Rgb(
        [(max_color * clamp(r, 0.0, 0.999)) as u16,
         (max_color * clamp(g, 0.0, 0.999)) as u16,
         (max_color * clamp(b, 0.0, 0.999)) as u16]
    ))
}

pub fn write_pixel_img_32bpp(x: u32, y: u32, pixel: vec3::Color, sample_count: u32, img: &mut ImageBuffer<Rgb<u32>, Vec<u32>>) {
    let (r,g,b) = convert_pixel(pixel, sample_count);

    let max_color = std::u32::MAX as f64 + 1.0;

    img.put_pixel(x, y, Rgb(
        [(max_color * clamp(r, 0.0, 0.999)) as u32,
         (max_color * clamp(g, 0.0, 0.999)) as u32,
         (max_color * clamp(b, 0.0, 0.999)) as u32]
    ))
}
