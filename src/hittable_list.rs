use crate::{aabb::*, hittable::*, ray::Ray};

use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new(object: Arc<dyn Hittable + Sync + Send>) -> HittableList {
        HittableList {
            objects: vec![object; 1],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                // Clone temp_rec contents into rec
                temp_rec.clone_into(rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> (bool, AABB) {
        if self.objects.is_empty() {
            return (false, AABB::new_e());
        }

        let mut temp_box: AABB;
        let mut out_box: AABB = AABB::new_e();
        let mut first_box = true;

        for object in self.objects.iter() {
            let (b, ttemp_box) = object.bounding_box(t0, t1);
            temp_box = ttemp_box;
            if !b {
                return (false, AABB::new_e());
            }
            out_box = if first_box {
                temp_box
            } else {
                surrounding_box(&out_box, &temp_box)
            };
            first_box = false;
        }

        (true, out_box)
    }
}
