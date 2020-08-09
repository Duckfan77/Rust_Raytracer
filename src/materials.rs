use crate::{
    ray::Ray,
    hittable::*,
    vec3::*,
};
use std::fmt::Debug;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

impl Debug for dyn Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Material")
    }
}


pub struct NoHit{
    a: u32
}

impl NoHit{
    pub fn new() -> NoHit {
        NoHit {a: 0}
    }
}

impl Material for NoHit {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color, _scattered: &mut Ray) -> bool {
        return false;
    }
}

