use crate::{
    vec3::*,
};
use std::fmt::Debug;

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