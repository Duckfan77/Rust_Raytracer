use crate::{
    vec3::*,
    perlin::Perlin,
    util::*,
};
use std::fmt::Debug;
use std::rc::Rc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

impl Debug for dyn Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Texture")
    }
}

pub struct SolidColor {
    color_val: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> SolidColor {
        SolidColor {color_val: c}
    }

    pub fn new_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {color_val: Color::new(r, g, b)}
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.color_val
    }
}


pub struct CheckerTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(t0: &Rc<dyn Texture>, t1: &Rc<dyn Texture>) -> CheckerTexture {
        CheckerTexture {even: Rc::clone(t0), odd: Rc::clone(t1)}
    }

    pub fn new_clr(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {even: Rc::new(SolidColor::new(c1)), odd: Rc::new(SolidColor::new(c2))}
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines = f64::sin(10.0*p.x()) * f64::sin(10.0*p.y()) * f64::sin(10.0*p.z());
        if sines < 0.0 {
            return self.odd.value(u, v, p)
        } else {
            return self.even.value(u, v, p)
        }
    }
}


pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
    pub color: Color,
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {noise: Perlin::new(), scale: 1.0, color: Color::new(1.0, 1.0, 1.0)}
    }

    pub fn new_sc(sc: f64) -> NoiseTexture {
        NoiseTexture {noise: Perlin::new(), scale: sc, color: Color::new(1.0, 1.0, 1.0)}
    }

    pub fn new_sc_clr(scale: f64, color: Color) -> NoiseTexture {
        NoiseTexture {noise: Perlin::new(), scale: scale, color: color}
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        return self.color * 0.5 * (1.0 + self.noise.noise(&(self.scale * *p)))
    }
}


pub struct TurbNoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
    pub color: Color,
}

impl TurbNoiseTexture {
    pub fn new() -> TurbNoiseTexture {
        TurbNoiseTexture {noise: Perlin::new(), scale: 1.0, color: Color::new(1.0, 1.0, 1.0)}
    }

    pub fn new_sc(sc: f64) -> TurbNoiseTexture {
        TurbNoiseTexture {noise: Perlin::new(), scale: sc, color: Color::new(1.0, 1.0, 1.0)}
    }

    pub fn new_sc_clr(scale: f64, color: Color) -> TurbNoiseTexture {
        TurbNoiseTexture {noise: Perlin::new(), scale: scale, color: color}
    }
}

impl Texture for TurbNoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        return self.color * self.noise.turb(&(self.scale * *p), 7);
    }
}


pub struct MarbleNoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
    pub color: Color,
}

impl MarbleNoiseTexture {
    pub fn new() -> MarbleNoiseTexture {
        MarbleNoiseTexture {noise: Perlin::new(), scale: 1.0, color: Color::new(1.0, 1.0, 1.0)}
    }

    pub fn new_sc(sc: f64) -> MarbleNoiseTexture {
        MarbleNoiseTexture {noise: Perlin::new(), scale: sc, color: Color::new(1.0, 1.0, 1.0)}
    }

    pub fn new_sc_clr(scale: f64, color: Color) -> MarbleNoiseTexture {
        MarbleNoiseTexture {noise: Perlin::new(), scale: scale, color: color}
    }
}

impl Texture for MarbleNoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        return self.color * 0.5 * (1.0 + f64::sin(self.scale*p.z() + 10.0*self.noise.turb(p, 7)));
    }
}

pub struct DualMarbleNoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
    pub color1: Color,
    pub color2: Color,
    pub weight: f64,
}

impl DualMarbleNoiseTexture {
    pub fn new() -> DualMarbleNoiseTexture {
        DualMarbleNoiseTexture {noise: Perlin::new(), scale: 1.0, color1: Color::new(1.0, 1.0, 1.0), color2: Color::new_e(), weight: 1.0}
    }

    pub fn new_sc(sc: f64) -> DualMarbleNoiseTexture {
        DualMarbleNoiseTexture {noise: Perlin::new(), scale: sc, color1: Color::new(1.0, 1.0, 1.0), color2: Color::new_e(), weight: 1.0}
    }

    pub fn new_sc_clr(scale: f64, color1: Color, color2: Color) -> DualMarbleNoiseTexture {
        DualMarbleNoiseTexture {noise: Perlin::new(), scale: scale, color1: color1, color2: color2, weight: 1.0}
    }

    pub fn new_sc_clr_weight(scale: f64, color1: Color, color2: Color, weight: f64) -> DualMarbleNoiseTexture {
        DualMarbleNoiseTexture {noise: Perlin::new(), scale: scale, color1: color1, color2: color2, weight: weight}
    }
}

impl Texture for DualMarbleNoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        let s = self.weight * 0.5 * (1.0 + f64::sin(self.scale*p.z() + 10.0*self.noise.turb(p, 7)));
        return (self.color1 * s) + (self.color2 * (1.0-s))
    }
}

const BYTES_PER_PIXEL: i32 = 3;

pub struct ImageTexture {
    width: i32,
    height: i32,
    bytes_per_scanline: i32,
    data: image::RgbImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        let mut width = 0;
        let mut height = 0;
        let bytes: i32;
        let mut data = image::RgbImage::new(width as u32, height as u32);

        let r = image::open(filename);

        match r {
            Ok(v) => {
                match v.as_rgb8() {
                    None => eprint!("Error: Could not convert to rgb8 image"),
                    Some (i) => {
                        height = i.height() as i32;
                        width = i.width() as i32;
                        data = i.clone();
                    }
                }
            },
            Err(_) => {
                eprint!("ERROR: Could not load texture image file '{}'.\n", filename);
                width = -1;
                height = -1;
            }
        }

        bytes = BYTES_PER_PIXEL * width;

        ImageTexture {
            width: width,
            height: height,
            bytes_per_scanline: bytes,
            data: data
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point) -> Color{
        //If no texture data, return solid cyan as debugging tool
        if self.height == -1 && self.width == -1 {
            return Color::new(0.0, 1.0, 1.0)
        }

        //Clamp coordinates to [0,1] x [1,0]
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0); //flip v to image coordinates

        let mut i = (u * self.width as f64) as i32;
        let mut j = (v * self.height as f64) as i32;

        //clamp int mapping, actual coordinates should be less than 1.0
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale  = 1.0/255.0;
        let pixel = self.data.get_pixel(i as u32, j as u32);

        Color::new(pixel[0] as f64 * color_scale,
                   pixel[1] as f64 * color_scale,
                   pixel[2] as f64 * color_scale,)
    }
}