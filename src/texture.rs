use crate::{
    vec3::*,
    perlin::Perlin,
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
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {noise: Perlin::new(), scale: 1.0}
    }

    pub fn new_sc(sc: f64) -> NoiseTexture {
        NoiseTexture {noise: Perlin::new(), scale: sc}
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        return Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(self.scale * *p)))
    }
}


pub struct TurbNoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl TurbNoiseTexture {
    pub fn new() -> TurbNoiseTexture {
        TurbNoiseTexture {noise: Perlin::new(), scale: 1.0}
    }

    pub fn new_sc(sc: f64) -> TurbNoiseTexture {
        TurbNoiseTexture {noise: Perlin::new(), scale: sc}
    }
}

impl Texture for TurbNoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        return Color::new(1.0, 1.0, 1.0) * self.noise.turb(&(self.scale * *p), 7);
    }
}


pub struct MarbleNoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl MarbleNoiseTexture {
    pub fn new() -> MarbleNoiseTexture {
        MarbleNoiseTexture {noise: Perlin::new(), scale: 1.0}
    }

    pub fn new_sc(sc: f64) -> MarbleNoiseTexture {
        MarbleNoiseTexture {noise: Perlin::new(), scale: sc}
    }
}

impl Texture for MarbleNoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        return Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + f64::sin(self.scale*p.z() + 10.0*self.noise.turb(p, 7)));
    }
}