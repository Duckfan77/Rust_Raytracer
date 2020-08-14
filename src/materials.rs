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
        return (false, Ray::new(&Point::new_e(), &Vec3::new_e(), 0.0), Color::new_e())
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Ray, Color){
        let scatter_dir = rec.normal + random_unit_vector();
        let scat = Ray::new(&rec.p, &scatter_dir, r_in.time());
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
        let scattered = Ray::new(&rec.p, &(reflected + self.fuzz*random_in_unit_sphere()), r_in.time());
        let atten = self.albedo;
        return (dot(scattered.direction(), rec.normal) > 0.0, scattered, atten)
    }
}


pub struct Dialectric {
    pub ref_idx: f64,
}

impl Dialectric {
    pub fn new (ri: f64) -> Dialectric {
        Dialectric {ref_idx: ri}
    }

    fn schlick(cos: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0*r0;
        return r0 + (1.0-r0) * (1.0 - cos).powf(5.0);
    }
}

impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Ray, Color) {
        let aten = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat: f64 = if rec.front_face {1.0 / self.ref_idx} else {self.ref_idx};

        let unit_dir = unit_vector(r_in.direction());

        let temp = dot(-unit_dir, rec.normal);
        let cos_theta = if temp < 1.0 {temp} else {1.0};
        let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);
        if etai_over_etat * sin_theta > 1.0 {
            // Must reflect
            let refl = reflect(&unit_dir, &rec.normal);
            let scattered = Ray::new(&rec.p, &refl, r_in.time());
            return (true, scattered, aten)
        }
        // Can refract
        let reflect_prob = Dialectric::schlick(cos_theta, etai_over_etat);
        if crate::util::random_double() < reflect_prob {
            let refl = reflect(&unit_dir, &rec.normal);
            let scattered = Ray::new(&rec.p, &refl, r_in.time());
            return (true, scattered, aten)
        }

        let refr = refract(&unit_dir, &rec.normal, etai_over_etat);
        let scattered = Ray::new(&rec.p, &refr, r_in.time());

        (true, scattered, aten)
    }
}