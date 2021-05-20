use crate::{aabb::AABB, hittable::*, materials::*, ray::Ray, texture::*, util::*, vec3::*};

use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable + Sync + Send>,
    phase_funct: Arc<dyn Material + Sync + Send>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new_txtr(
        b: Arc<dyn Hittable + Sync + Send>,
        d: f64,
        a: Arc<dyn Texture + Sync + Send>,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_funct: Arc::new(Isotropic::new_txtr(a)),
            neg_inv_density: -1.0 / d,
        }
    }

    pub fn new(b: Arc<dyn Hittable + Sync + Send>, d: f64, c: Color) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_funct: Arc::new(Isotropic::new(c)),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Print occasional samples when debugging. To enable, set enableDebug true
        #[allow(non_snake_case)]
        let enableDebug = false;
        let debugging = enableDebug && random_double() < 0.00001;

        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, rec1.t + 0.001, INFINITY, &mut rec2) {
            return false;
        }

        if debugging {
            eprint!("\nt0={}, t1={}\n", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_len = r.direction().length();
        let dist_inside_boundary = (rec2.t - rec1.t) * ray_len;
        let hit_dist = self.neg_inv_density * f64::log(random_double(), E);

        if hit_dist > dist_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_dist / ray_len;
        rec.p = r.at(rec.t);

        if debugging {
            eprint!(
                "hit_dist = {}\nrec.t = {}\nrec.p = {}\n",
                hit_dist, rec.t, rec.p
            );
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0); //Arbitrary, doesn't really apply
        rec.front_face = true; //Arbitrary, doesn't really apply
        rec.mat_ptr = Arc::clone(&self.phase_funct);

        true
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> (bool, AABB) {
        self.boundary.bounding_box(t0, t1)
    }
}
