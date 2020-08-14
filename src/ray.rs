use super::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray{
    orig: Point,
    dir: Vec3,
    tm: f64,
}

impl Ray{
    pub fn new(origin: &Point, direction: &Vec3, time: f64) -> Ray{
        Ray {orig: *origin, dir: *direction, tm: time}
    }

    pub fn origin(&self) -> Point{
        self.orig
    }

    pub fn direction(&self) -> Vec3{
        self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point{
        self.orig + t*self.dir
    }
}