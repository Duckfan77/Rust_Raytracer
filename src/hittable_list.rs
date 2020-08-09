use crate::{
    hittable::*,
    ray::Ray,
};

use std::rc::Rc;

pub struct HittableList{
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList{
    pub fn new(object: Rc<dyn Hittable>) -> HittableList {
        HittableList {objects: vec![object; 1]}
    }

    pub fn clear(&mut self){
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>){
        self.objects.push(object);
    }
}

impl Hittable for HittableList{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects{
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                // Clone temp_rec contents into rec
                temp_rec.clone_into(rec);
            }
        }

        return hit_anything;
    }
}