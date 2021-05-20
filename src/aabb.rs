//axis aligned bounding box

use crate::ray::Ray;
use crate::vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    min: Point,
    max: Point,
}

impl AABB {
    pub fn new_e() -> AABB {
        AABB {
            min: Point::new_e(),
            max: Point::new_e(),
        }
    }

    pub fn new(a: &Point, b: &Point) -> AABB {
        AABB { min: *a, max: *b }
    }

    pub fn min(&self) -> Point {
        self.min
    }

    pub fn max(&self) -> Point {
        self.max
    }

    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let t0 = f64::min(
                (self.min[a] - r.origin()[a]) / r.direction()[a],
                (self.max[a] - r.origin()[a]) / r.direction()[a],
            );
            let t1 = f64::max(
                (self.min[a] - r.origin()[a]) / r.direction()[a],
                (self.max[a] - r.origin()[a]) / r.direction()[a],
            );

            let tmin = f64::max(t0, tmin);
            let tmax = f64::min(t1, tmax);

            if tmax <= tmin {
                return false;
            }
        }

        return true;
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Point::new(
        f64::min(box0.min().x(), box1.min().x()),
        f64::min(box0.min().y(), box1.min().y()),
        f64::min(box0.min().z(), box1.min().z()),
    );
    let big = Point::new(
        f64::max(box0.max().x(), box1.max().x()),
        f64::max(box0.max().y(), box1.max().y()),
        f64::max(box0.max().z(), box1.max().z()),
    );

    return AABB::new(&small, &big);
}
