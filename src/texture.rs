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
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {noise: Perlin::new()}
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        return Color::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}