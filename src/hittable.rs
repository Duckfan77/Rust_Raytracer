use crate::vec3::*;
use crate::ray::Ray;
use std::rc::Rc;
use crate::materials::*;
use crate::aabb::*;

#[derive(Debug, Clone)]
pub struct HitRecord{
    pub p: Point,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
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
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn clone_into(&self, target: &mut HitRecord){
        target.p = self.p.clone();
        target.normal = self.normal.clone();
        target.mat_ptr = Rc::clone(&self.mat_ptr);
        target.t = self.t.clone();
        target.u = self.u.clone();
        target.v = self.v.clone();
        target.front_face = self.front_face.clone();
    }
}

pub trait Hittable{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64) -> (bool, AABB);
}
