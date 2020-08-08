use crate::vec3::*;
use crate::ray::Ray;

pub struct HitRecord{
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool;
}