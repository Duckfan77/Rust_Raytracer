use crate::{
    hittable::*,
    vec3::*,
    aarect::*,
    hittable_list::*,
    materials::*,
    aabb::AABB,
    ray::Ray,
};
use std::rc::Rc;

pub struct Box {
    box_min: Point,
    box_max: Point,
    sides: HittableList,
}

impl Box {
    pub fn new(p0: &Point, p1: &Point, mat: Rc<dyn Material>) -> Box {
        let box_min = *p0;
        let box_max = *p1;
        let mut sides = HittableList {objects: Vec::with_capacity(10)};

        sides.add(Rc::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), Rc::clone(&mat))));
        sides.add(Rc::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), Rc::clone(&mat))));

        sides.add(Rc::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), Rc::clone(&mat))));
        sides.add(Rc::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), Rc::clone(&mat))));

        sides.add(Rc::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), Rc::clone(&mat))));
        sides.add(Rc::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), Rc::clone(&mat))));

        Box {
            box_min: box_min,
            box_max: box_max,
            sides: sides,
        }
    }
}

impl Hittable for Box {
    fn bounding_box(&self, _t0: f64, _t1: f64) -> (bool, AABB) {
        let outbox = AABB::new(&self.box_min, &self.box_max);
        return (true, outbox)
    }

    fn hit(&self, r: &Ray, t0: f64, t1: f64, rec: &mut HitRecord) -> bool {
        return self.sides.hit(r, t0, t1, rec)
    }
}