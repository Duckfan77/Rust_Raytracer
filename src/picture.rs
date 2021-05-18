use image::{ImageBuffer,Rgb};
use std::fs::File;
use std::io::Write;
use crate::vec3::Color;
use crate::color;

arg_enum!{
    /**
     * Public Picture Type enum to inform which file type to write.
     *
     * Ppm: 8 bit per channel PPM file, must be writen strictly in order
     * Rgb8: 8 bit per channel PNG or JPG, depending on file name
     * Rgb16: 16 bit per channel PNG or JPG, depending on file name
     */
    #[derive(Debug, PartialEq)]
    pub enum PictureType {
        Ppm,
        Rgb8,
        Rgb16,
    }
}

#[derive(Debug)]
pub enum PictureErr {
    IOError {err: std::io::Error},
    InvalidArgs {err: String},
    ImgError {err: image::ImageError},
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
    /**
     * Creates new Picture
     *
     * aspect_ratio must be strictly positive, and must lead to a non-zero height when combined with width.
     * samples_per_pixel must be non-zero
     *
     * If outtype is Rgb8 or Rgb16, fname will determine image output type, either jpg or png.
     * If outtype is Ppm, will write header of file in constructor, and must be written directly in order, and cannot be backtracked. Writing a pixel or row to a Ppm type Picture is writing to disk directly.
     */
    pub fn new(width: u32, aspect_ratio: f64, samples_per_pixel: u32, fname: &String, outtype: PictureType) -> Result<Picture, PictureErr> {
        let height = (width as f64 / aspect_ratio) as u32;

        if aspect_ratio <= 0.0 {
            return Err(PictureErr::InvalidArgs{err: "aspect_ratio less or equal to 0 on creation of Picture".to_string()})
        }

        if width == 0 || height == 0 || samples_per_pixel == 0 {
            return Err(PictureErr::InvalidArgs{err: "image width, height, or samples_per_pixel invalid on creation of Picture".to_string()})
        }

        //Create Image
        let img = match outtype {
            PictureType::Ppm => {
                let mut file = match File::create(fname) {
                    Ok(f) => f,

                    Err(e) => return Err(PictureErr::IOError{err: e})
                };

                //write header of ppm to file
                match write!(file, "P3\n{} {}\n255\n", width, height){
                    Ok(_) => {},
                    Err(e) => return Err(PictureErr::IOError{err: e})
                }

                PictureBuf::Ppm {
                    file: file,
                    x: 0,
                    y: 0
                }
            }

            PictureType::Rgb8 => {
                PictureBuf::Rgb8 {
                    buf: ImageBuffer::new(width, height)
                }
            }

            PictureType::Rgb16 => {
                PictureBuf::Rgb16 {
                    buf: ImageBuffer::new(width, height)
                }
            }
        };

        Ok(Picture{
            width: width,
            aspect_ratio: aspect_ratio,
            samples: samples_per_pixel,
            fname: fname.clone(),
            img: img,
        })
    }

    /**
     * Writes full row of image
     *
     * Must have row size be equal to width of image
     *
     * Note for Ppm images:
     * If Ppm image is not at y equal to row_num and x equal to 0, will error.
     * Upon completion, Ppm image y is incremented by 1
     * Writes to disk at this step, Ppm pixels are not later mutable
     */
    pub fn write_row(&mut self, row: &Vec<Color>, row_num: u32) -> Result<(), PictureErr>{
        if row.len() != self.width as usize {
            return Err(PictureErr::InvalidArgs{err: "Row Length Does Not Match Image Width".to_string()});
        }

        if row_num >= self.height() {
            return Err(PictureErr::InvalidArgs{err: "Row beyond height of image".to_string()});
        }

        match &mut self.img {
            //write ppm row
            PictureBuf::Ppm{ref mut file, ref mut x, ref mut y} => {
                if *x != 0 || *y != row_num {
                    return Err(PictureErr::InvalidArgs{err: "Ppm file not at correct coordinates at start of write_row".to_string()})
                }

                for p in row {
                    color::write_color_ppm(file, *p, self.samples);
                }

                *y += 1;
            }

            //Write Rgb8 row
            PictureBuf::Rgb8{ref mut buf} => {
                for (col,p) in row.iter().enumerate() {
                    color::write_pixel_img_8bpc(col as u32, row_num, *p, self.samples, buf);
                }
            }

            //Write Rgb16 row
            PictureBuf::Rgb16{ref mut buf} => {
                for (col,p) in row.iter().enumerate() {
                    color::write_pixel_img_16bpc(col as u32, row_num, *p, self.samples, buf);
                }
            }
        };

        Ok(())
    }

    /**
     * Writes pixel to image
     *
     * Must be within bounds of image
     *
     * Note for Ppm images:
     * If Ppm image is not at x equal to row and y equal to col, will error.
     * Upon completion, Ppm image index incremented by 1, first y, wrapping around to 0 and incrementing x if end of row
     * Writes to disk at this step, Ppm pixels are not later mutable
     */
    pub fn write_pixel(&mut self, pixel: &Color, row: u32, col: u32) -> Result<(), PictureErr>{
        if col >= self.width {
            return Err(PictureErr::InvalidArgs{err: "Column beyond width of image".to_string()});
        }

        if row >= self.height() {
            return Err(PictureErr::InvalidArgs{err: "Row beyond height of image".to_string()});
        }

        match &mut self.img {
            PictureBuf::Ppm{ref mut file, ref mut x, ref mut y} => {
                if *x != col || *y != row {
                    return Err(PictureErr::InvalidArgs{err: "Ppm file not at correct coordinates at start of write_pixel".to_string()})
                }

                color::write_color_ppm(file, *pixel, self.samples);

                *x += 1;
                if *x == self.width {
                    *x = 0;
                    *y += 1;
                }
            }

            PictureBuf::Rgb8{ref mut buf} => {
                color::write_pixel_img_8bpc(col, row, *pixel, self.samples, buf);
            }

            PictureBuf::Rgb16{ref mut buf} => {
                color::write_pixel_img_16bpc(col, row, *pixel, self.samples, buf);
            }
        }

        Ok(())
    }

    /**
     * Saves data to file for Rgb8 and Rgb16 image types. Ppm is actively saving to disk at pixel writing time and this will simply flush the buffers.
     */
    pub fn save(&mut self) -> Result<(), PictureErr> {
        match &mut self.img {
            //flush Ppm
            PictureBuf::Ppm{ref mut file, ..} => {
                match file.flush() {
                    Ok(_) => {}

                    Err(e) => return Err(PictureErr::IOError{err: e})
                }
            }

            PictureBuf::Rgb8{ref mut buf} => {
                match buf.save(&self.fname) {
                    Ok(_) => {},

                    Err(e) => return Err(PictureErr::ImgError{err: e})
                }
            }

            PictureBuf::Rgb16{ref mut buf} => {
                match buf.save(&self.fname) {
                    Ok(_) => {},

                    Err(e) => return Err(PictureErr::ImgError{err: e})
                }
            }
        };

        Ok(())
    }

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

    pub fn ppm_x(&self) -> Option<u32> {
        match self.img {
            PictureBuf::Ppm{file: _, x, ..} => Some(x),

            _ => None
        }
    }

    pub fn ppm_y(&self) -> Option<u32> {
        match self.img {
            PictureBuf::Ppm{file: _, x: _, y} => Some(y),

            _ => None
        }
    }
}
