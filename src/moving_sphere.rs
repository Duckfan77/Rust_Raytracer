use crate::{aabb::*, hittable::*, materials::*, ray::Ray, vec3::*};
use std::f64;
use std::sync::Arc;

#[derive(Clone)]
pub struct MovingSphere {
    center0: Point,
    center1: Point,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: Arc<dyn Material + Sync + Send>,
}

impl MovingSphere {
    pub fn new(
        cen0: Point,
        cen1: Point,
        t0: f64,
        t1: f64,
        r: f64,
        m: Arc<dyn Material + Sync + Send>,
    ) -> MovingSphere {
        MovingSphere {
            center0: cen0,
            center1: cen1,
            time0: t0,
            time1: t1,
            radius: r,
            mat_ptr: Arc::clone(&m),
        }
    }

    pub fn center(&self, time: f64) -> Point {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        //Calculate 1/4 of the discriminant, b/2 * b/2 = b/4, allowing us to remove the 4 from the - 4ac, this gets caught by clippy, isn't a bug
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = f64::sqrt(discriminant);

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center(r.time())) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = Arc::clone(&self.mat_ptr);
                return true;
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center(r.time())) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = Arc::clone(&self.mat_ptr);
                return true;
            }
        }

        false
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> (bool, AABB) {
        let box0 = AABB::new(
            &(self.center(t0) - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center(t0) + Vec3::new(self.radius, self.radius, self.radius)),
        );
        let box1 = AABB::new(
            &(self.center(t1) - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center(t1) + Vec3::new(self.radius, self.radius, self.radius)),
        );

        let out_box = surrounding_box(&box0, &box1);
        (true, out_box)
    }
}
