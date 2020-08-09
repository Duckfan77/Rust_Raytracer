use crate::{
    ray::Ray,
    hittable::*,
    vec3::*,
};
use std::fmt::Debug;

pub trait Material {
    /// Scatters an incoming ray according to the Material implementing it and the HitRecord.
    ///
    /// Returns bool for success, Ray containing the scattered ray, and Color containing the attenuation
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Ray, Color);
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
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> (bool, Ray, Color) {
        return (false, Ray::new(&Point::new_e(), &Vec3::new_e()), Color::new_e())
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian {albedo: a}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> (bool, Ray, Color){
        let scatter_dir = rec.normal + random_unit_vector();
        let scat = Ray::new(&rec.p, &scatter_dir);
        let atten = self.albedo;
        return (true, scat, atten)
    }
}


pub struct Metal{
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) ->Metal {
        Metal {albedo: a, fuzz: if f < 1.0 {f} else {1.0} }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Ray, Color) {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        let scattered = Ray::new(&rec.p, &(reflected + self.fuzz*random_in_unit_sphere()));
        let atten = self.albedo;
        return (dot(scattered.direction(), rec.normal) > 0.0, scattered, atten)
    }
}