use crate::{
    hittable::*,
    materials::*,
    ray::Ray,
    aabb::*,
    vec3::*,
};

use std::sync::Arc;

pub struct XYRect {
    mat_ptr: Arc<dyn Material + Sync + Send>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<dyn Material + Sync + Send>) -> XYRect {
        XYRect {
            mat_ptr: mat,
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1,
            k: k,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t0: f64, t1: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t0 || t > t1 {
            return false
        }

        let x = r.origin().x() + t*r.direction().x();
        let y = r.origin().y() + t*r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false
        }

        rec.u = (x-self.x0)/(self.x1 - self.x0);
        rec.v = (y-self.y0)/(self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Arc::clone(&self.mat_ptr);
        rec.p = r.at(t);
        return true
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> (bool, AABB) {
        // The bounding box must be non-zero width in all dimensions, so bad the z dimension a small amount
        let outbox = AABB::new(&Point::new(self.x0, self.y0, self.k-0.0001), &Point::new(self.x1, self.y1, self.k+0.001));
        return (true, outbox)
    }
}


pub struct XZRect {
    mat_ptr: Arc<dyn Material + Sync + Send>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material + Sync + Send>) -> XZRect {
        XZRect {
            mat_ptr: mat,
            x0: x0,
            x1: x1,
            z0: z0,
            z1: z1,
            k: k,
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: &Ray, t0: f64, t1: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t0 || t > t1 {
            return false
        }

        let x = r.origin().x() + t*r.direction().x();
        let z = r.origin().z() + t*r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false
        }

        rec.u = (x-self.x0)/(self.x1 - self.x0);
        rec.v = (z-self.z0)/(self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Arc::clone(&self.mat_ptr);
        rec.p = r.at(t);
        return true
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> (bool, AABB) {
        // The bounding box must be non-zero width in all dimensions, so bad the z dimension a small amount
        let outbox = AABB::new(&Point::new(self.x0, self.k-0.0001, self.z0), &Point::new(self.x1, self.k+0.001, self.z1));
        return (true, outbox)
    }
}


pub struct YZRect {
    mat_ptr: Arc<dyn Material + Sync + Send>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material + Sync + Send>) -> YZRect {
        YZRect {
            mat_ptr: mat,
            y0: y0,
            y1: y1,
            z0: z0,
            z1: z1,
            k: k,
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t0: f64, t1: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t0 || t > t1 {
            return false
        }

        let y = r.origin().y() + t*r.direction().y();
        let z = r.origin().z() + t*r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false
        }

        rec.u = (y-self.y0)/(self.y1 - self.y0);
        rec.v = (z-self.z0)/(self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Arc::clone(&self.mat_ptr);
        rec.p = r.at(t);
        return true
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> (bool, AABB) {
        // The bounding box must be non-zero width in all dimensions, so bad the z dimension a small amount
        let outbox = AABB::new(&Point::new(self.k-0.0001, self.y0, self.z0), &Point::new(self.k+0.001, self.y1, self.z1));
        return (true, outbox)
    }
}