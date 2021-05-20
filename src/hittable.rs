use crate::aabb::*;
use crate::materials::*;
use crate::ray::Ray;
use crate::util::*;
use crate::vec3::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material + Sync + Send>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

    pub fn new() -> HitRecord {
        HitRecord {
            p: Point::new_e(),
            normal: Vec3::new_e(),
            mat_ptr: Arc::new(NoHit::new()),
            t: 0.0,
            u: 0.0,
            v: 0.5,
            front_face: false,
        }
    }

    pub fn clone_into(&self, target: &mut HitRecord) {
        target.p = self.p;
        target.normal = self.normal;
        target.mat_ptr = Arc::clone(&self.mat_ptr);
        target.t = self.t;
        target.u = self.u;
        target.v = self.v;
        target.front_face = self.front_face;
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64) -> (bool, AABB);
}

pub struct Translate {
    ptr: Arc<dyn Hittable + Sync + Send>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable + Sync + Send>, displacement: &Vec3) -> Translate {
        Translate {
            ptr: p,
            offset: *displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(&(r.origin() - self.offset), &r.direction(), r.time());
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        let n = rec.normal;
        rec.set_face_normal(&moved_r, &n);

        true
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> (bool, AABB) {
        let (b, outbox) = self.ptr.bounding_box(t0, t1);
        if !b {
            return (false, AABB::new_e());
        }

        let outbox = AABB::new(&(outbox.min() + self.offset), &(outbox.max() + self.offset));

        (true, outbox)
    }
}

pub struct RotateX {
    ptr: Arc<dyn Hittable + Sync + Send>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}

impl RotateX {
    pub fn new(p: Arc<dyn Hittable + Sync + Send>, angle: f64) -> RotateX {
        let ptr = p;
        let sin: f64;
        let cos: f64;

        let rads = degs_to_rads(angle);
        sin = f64::sin(rads);
        cos = f64::cos(rads);
        let (hasbox, bbox) = ptr.bounding_box(0.0, 1.0);

        let mut min = Point::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                    let newy = cos * y + sin * z;
                    let newz = -sin * y + cos * z;

                    let tester = Point::new(x, newy, newz);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::new(&min, &max);

        RotateX {
            ptr,
            sin_theta: sin,
            cos_theta: cos,
            hasbox,
            bbox,
        }
    }
}

impl Hittable for RotateX {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut dir = r.direction();

        origin[1] = self.cos_theta * r.origin().y() - self.sin_theta * r.origin().z();
        origin[2] = self.sin_theta * r.origin().y() + self.cos_theta * r.origin().z();

        dir[1] = self.cos_theta * r.direction().y() - self.sin_theta * r.direction().z();
        dir[2] = self.sin_theta * r.direction().y() + self.cos_theta * r.direction().z();

        let rotated_r = Ray::new(&origin, &dir, r.time());

        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut n = rec.normal;

        p[1] = self.cos_theta * rec.p.y() + self.sin_theta * rec.p.z();
        p[2] = -self.sin_theta * rec.p.y() + self.cos_theta * rec.p.z();

        n[1] = self.cos_theta * rec.normal.y() + self.sin_theta * rec.normal.z();
        n[2] = -self.sin_theta * rec.normal.y() + self.cos_theta * rec.normal.z();

        rec.p = p;
        rec.set_face_normal(&rotated_r, &n);

        true
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> (bool, AABB) {
        (self.hasbox, self.bbox)
    }
}

pub struct RotateY {
    ptr: Arc<dyn Hittable + Sync + Send>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable + Sync + Send>, angle: f64) -> RotateY {
        let ptr = p;
        let sin: f64;
        let cos: f64;

        let rads = degs_to_rads(angle);
        sin = f64::sin(rads);
        cos = f64::cos(rads);
        let (hasbox, bbox) = ptr.bounding_box(0.0, 1.0);

        let mut min = Point::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                    let newx = cos * x + sin * z;
                    let newz = -sin * x + cos * z;

                    let tester = Point::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::new(&min, &max);

        RotateY {
            ptr,
            sin_theta: sin,
            cos_theta: cos,
            hasbox,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut dir = r.direction();

        origin[0] = self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z();
        origin[2] = self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z();

        dir[0] = self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z();
        dir[2] = self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z();

        let rotated_r = Ray::new(&origin, &dir, r.time());

        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut n = rec.normal;

        p[0] = self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z();
        p[2] = -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z();

        n[0] = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
        n[2] = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

        rec.p = p;
        rec.set_face_normal(&rotated_r, &n);

        true
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> (bool, AABB) {
        (self.hasbox, self.bbox)
    }
}

pub struct RotateZ {
    ptr: Arc<dyn Hittable + Sync + Send>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}

impl RotateZ {
    pub fn new(p: Arc<dyn Hittable + Sync + Send>, angle: f64) -> RotateZ {
        let ptr = p;
        let sin: f64;
        let cos: f64;

        let rads = degs_to_rads(angle);
        sin = f64::sin(rads);
        cos = f64::cos(rads);
        let (hasbox, bbox) = ptr.bounding_box(0.0, 1.0);

        let mut min = Point::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                    let newx = cos * x + sin * y;
                    let newy = -sin * x + cos * y;

                    let tester = Point::new(newx, newy, z);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::new(&min, &max);

        RotateZ {
            ptr,
            sin_theta: sin,
            cos_theta: cos,
            hasbox,
            bbox,
        }
    }
}

impl Hittable for RotateZ {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut dir = r.direction();

        origin[0] = self.cos_theta * r.origin().x() - self.sin_theta * r.origin().y();
        origin[1] = self.sin_theta * r.origin().x() + self.cos_theta * r.origin().y();

        dir[0] = self.cos_theta * r.direction().x() - self.sin_theta * r.direction().y();
        dir[1] = self.sin_theta * r.direction().x() + self.cos_theta * r.direction().y();

        let rotated_r = Ray::new(&origin, &dir, r.time());

        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut n = rec.normal;

        p[0] = self.cos_theta * rec.p.x() + self.sin_theta * rec.p.y();
        p[1] = -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.y();

        n[0] = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.y();
        n[1] = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.y();

        rec.p = p;
        rec.set_face_normal(&rotated_r, &n);

        true
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> (bool, AABB) {
        (self.hasbox, self.bbox)
    }
}
