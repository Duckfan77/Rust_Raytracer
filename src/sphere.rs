use crate::{aabb::AABB, hittable::*, materials::*, ray::Ray, util::*, vec3::*};
use std::f64;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    center: Point,
    radius: f64,
    mat_ptr: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(cen: Point, r: f64, m: Arc<dyn Material + Sync + Send>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }
}

///Vec3 -> u, v
pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
    let phi = f64::atan2(p.z(), p.x());
    let theta = f64::asin(p.y());
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    return (u, v);
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = f64::sqrt(discriminant);

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(&r, &outward_normal);
                let (u, v) = get_sphere_uv(&((rec.p - self.center) / self.radius));
                rec.u = u;
                rec.v = v;
                rec.mat_ptr = Arc::clone(&self.mat_ptr);
                return true;
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(&r, &outward_normal);
                let (u, v) = get_sphere_uv(&((rec.p - self.center) / self.radius));
                rec.u = u;
                rec.v = v;
                rec.mat_ptr = Arc::clone(&self.mat_ptr);
                return true;
            }
        }

        return false;
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> (bool, AABB) {
        let out_box = AABB::new(
            &(self.center - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center + Vec3::new(self.radius, self.radius, self.radius)),
        );
        return (true, out_box);
    }
}
