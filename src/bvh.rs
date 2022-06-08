use crate::{aabb::AABB, aabb::*, hittable::*, hittable_list::HittableList, ray::Ray, util::*};
use std::cmp::Ordering;
use std::f64;
use std::sync::Arc;

pub struct BvhNode {
    pub left: Arc<dyn Hittable + Sync + Send>,
    pub right: Arc<dyn Hittable + Sync + Send>,
    pub bbox: AABB,
}

impl BvhNode {
    pub fn new_l(list: &mut HittableList, time0: f64, time1: f64) -> BvhNode {
        let len = list.objects.len();
        BvhNode::new(&mut list.objects, 0, len, time0, time1)
    }

    pub fn new(
        objects: &mut Vec<Arc<dyn Hittable + Sync + Send>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let left: Arc<dyn Hittable + Sync + Send>;
        let right: Arc<dyn Hittable + Sync + Send>;

        let axis = random_int_range(0, 2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span: usize = end - start;

        if object_span == 1 {
            left = Arc::clone(&objects[start]);
            right = Arc::clone(&objects[start]);
        } else if object_span == 2 {
            if comparator(Arc::clone(&objects[start]), Arc::clone(&objects[start + 1]))
                == Ordering::Less
            {
                left = Arc::clone(&objects[start]);
                right = Arc::clone(&objects[start + 1]);
            } else {
                left = Arc::clone(&objects[start + 1]);
                right = Arc::clone(&objects[start]);
            }
        } else {
            objects.sort_unstable_by(|a, b| comparator(Arc::clone(a), Arc::clone(b)));

            let mid = start + object_span / 2;
            left = Arc::new(BvhNode::new(objects, start, mid, time0, time1));
            right = Arc::new(BvhNode::new(objects, mid, end, time0, time1));
        }

        let (abool, box_left) = left.bounding_box(time0, time1);
        let (bbool, box_right) = right.bounding_box(time0, time1);

        if !abool || !bbool {
            eprintln!(
                "No bounding box in BvhNode constructor. L:{:?}, R:{:?}",
                box_left, box_right
            );
        }

        let bbox = surrounding_box(&box_left, &box_right);

        BvhNode { left, right, bbox }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> (bool, AABB) {
        (true, self.bbox)
    }
}

pub fn box_compare(
    a: Arc<dyn Hittable + Sync + Send>,
    b: Arc<dyn Hittable + Sync + Send>,
    axis: usize,
) -> Ordering {
    let (abool, box_a) = a.bounding_box(0.0, 0.0);
    let (bbool, box_b) = b.bounding_box(0.0, 0.0);

    if !abool || !bbool {
        eprintln!("No bounding box in BvhNode constructor.");
    }

    match f64::partial_cmp(&box_a.min()[axis], &box_b.min()[axis]) {
        None => Ordering::Less,
        Some(i) => i,
    }
}

pub fn box_x_compare(
    a: Arc<dyn Hittable + Sync + Send>,
    b: Arc<dyn Hittable + Sync + Send>,
) -> Ordering {
    box_compare(a, b, 0)
}

pub fn box_y_compare(
    a: Arc<dyn Hittable + Sync + Send>,
    b: Arc<dyn Hittable + Sync + Send>,
) -> Ordering {
    box_compare(a, b, 1)
}

pub fn box_z_compare(
    a: Arc<dyn Hittable + Sync + Send>,
    b: Arc<dyn Hittable + Sync + Send>,
) -> Ordering {
    box_compare(a, b, 2)
}
