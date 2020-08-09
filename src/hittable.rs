use crate::vec3::*;
use crate::ray::Ray;
use std::rc::Rc;
use crate::materials::*;

#[derive(Debug, Clone)]
pub struct HitRecord{
    pub p: Point,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord{
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3){
        self.front_face = dot(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-*outward_normal};
    }

    pub fn new() -> HitRecord {
        HitRecord {
            p: Point::new_e(),
            normal: Vec3::new_e(),
            mat_ptr: Rc::new(NoHit::new()),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
