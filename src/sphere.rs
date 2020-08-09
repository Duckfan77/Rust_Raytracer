use crate::{
    hittable::*,
    vec3::*,
    ray::Ray,
    materials::*,
};
use std::f64;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Sphere{
    center: Point,
    radius: f64,
    mat_ptr: Rc<dyn Material>,
}

impl Sphere{
    pub fn new(cen: Point, r: f64, m: Rc<dyn Material>) -> Sphere{
        Sphere {center: cen, radius: r, mat_ptr: m}
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0.0{
            let root = f64::sqrt(discriminant);

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(&r, &outward_normal);
                rec.mat_ptr = Rc::clone(&self.mat_ptr);
                return true
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(&r, &outward_normal);
                rec.mat_ptr = Rc::clone(&self.mat_ptr);
                return true
            }
        }

        return false
    }
}