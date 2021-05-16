use image::{ImageBuffer,Rgb};
use std::fs::File;
use crate::vec3::Color;

/**
 * Public Picture Type enum to inform which file type to write.
 *
 * Ppm: 8 bit per channel PPM file
 * Rgb8: 8 bit per channel PNG or JPG, depending on file name
 * Rgb16: 16 bit per channel PNG or JPG, depending on file name
 */
pub enum PictureType {
    Ppm,
    Rgb8,
    Rgb16,
}

/**
 * Private PictureBuf to store internals required for each image type
 */
enum PictureBuf {
    Ppm {file: File, x: u32, y: u32},
    Rgb8 {buf: ImageBuffer<Rgb<u8>, Vec<u8>>},
    Rgb16 {buf: ImageBuffer<Rgb<u16>, Vec<u16>>},
}

pub struct Picture{
    width: u32,
    aspect_ratio: f64,
    samples: u32,
    fname: String,
    img: PictureBuf,
}

impl Picture{
    //Getters for key values
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        (self.width as f64 / self.aspect_ratio) as u32
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn samples_per_pixel(&self) -> u32 {
        self.samples
    }
}
