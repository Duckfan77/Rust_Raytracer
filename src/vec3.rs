use std::ops::*;
use std::fmt;
use std::f64;
use crate::util::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3{
    e0: f64,
    e1: f64,
    e2: f64,
}

#[allow(dead_code)]
impl Vec3{
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        return Vec3 {e0: e0,
                     e1: e1,
                     e2: e2,
                    };
    }

    pub fn new_e() -> Vec3 {
        return Vec3 {e0: 0.0,
                     e1: 0.0,
                     e2: 0.0,
                    };
    }

    pub fn x(self) -> f64{
        self.e0
    }

    pub fn y(self) -> f64{
        self.e1
    }

    pub fn z(self) -> f64{
        self.e2
    }

    pub fn length(self) -> f64{
        return f64::sqrt(self.length_squared());
    }

    pub fn length_squared(self) -> f64{
        return self.e0*self.e0 + self.e1*self.e1 + self.e2*self.e2;
    }
}

pub fn random() -> Vec3 {
    Vec3::new(random_double(), random_double(), random_double())
}

pub fn random_range(min: f64, max: f64) -> Vec3 {
    Vec3::new(random_double_range(min, max), random_double_range(min, max), random_double_range(min, max))
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {return p;}
    }
}

pub fn random_unit_vector() -> Vec3 {
    let a = random_double_range(0.0, 2.0*PI);
    let z = random_double_range(-1.0, 1.0);
    let r = f64::sqrt(1.0 - z*z);
    return Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {return p;}
    }
}

/// reflects a vector impacting a mirrored surface with normal n. The normal is assumed to be a unit vector. Returns the reflected vector.
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3{
    return *v - 2.0*dot(*v, *n) * *n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3{
    let cos_theta = dot(-*uv, *n);
    let r_out_perp = etai_over_etat * (*uv + cos_theta**n);
    let r_out_parl = -f64::sqrt( (1.0 - r_out_perp.length_squared()).abs()) * *n;
    return r_out_perp + r_out_parl
}

pub fn dot(u: Vec3, v: Vec3) -> f64{
    return u.e0 * v.e0
         + u.e1 * v.e1
         + u.e2 * v.e2;
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3{
    return Vec3::new(u.e1 * v.e2 - u.e2 * v.e1,
                     u.e2 * v.e0 - u.e0 * v.e2,
                     u.e0 * v.e1 - u.e1 * v.e0);
}

pub fn unit_vector(v: Vec3) -> Vec3{
    return v / v.length();
}

impl Neg for Vec3{
    type Output = Vec3;

    fn neg(self) -> Self::Output{
        return Vec3::new(-self.e0, -self.e1, -self.e2);
    }
}

impl Index<usize> for Vec3{
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.e0,
            1 => &self.e1,
            2 => &self.e1,
            _ => &0.0,
        }
    }
}

impl IndexMut<usize> for Vec3{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output{
        match index {
            0 => &mut self.e0,
            1 => &mut self.e1,
            2 => &mut self.e2,
            _ => &mut self.e2,
        }
    }
}

impl AddAssign for Vec3{
    fn add_assign(&mut self, rhs: Self){
        self.e0 += rhs.e0;
        self.e1 += rhs.e1;
        self.e2 += rhs.e2;
    }
}

impl MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, rhs: f64){
        self.e0 *= rhs;
        self.e1 *= rhs;
        self.e2 *= rhs;
    }
}

impl DivAssign<f64> for Vec3{
    fn div_assign(&mut self, rhs: f64){
        *self *= 1.0/rhs;
    }
}

impl fmt::Display for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} {} {}", self.e0, self.e1, self.e2)
    }
}

impl Add for Vec3{
    type Output = Self;

    fn add(self, v: Vec3) -> Self::Output{
        return Vec3::new(self.e0 + v.e0,
                         self.e1 + v.e1,
                         self.e2 + v.e2)
    }
}

impl Sub for Vec3{
    type Output = Self;

    fn sub(self, v: Vec3) -> Self::Output{
        return Vec3::new(self.e0 - v.e0,
                         self.e1 - v.e1,
                         self.e2 - v.e2,);
    }
}

impl Mul for Vec3{
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3{
        Vec3::new(self.e0 * v.e0, self.e1 * v.e1, self.e2 * v.e2)
    }
}

impl Mul<Vec3> for f64{
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3{
        return Vec3::new(self*v.e0, self*v.e1, self*v.e2);
    }
}

impl Mul<f64> for Vec3{
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3{
        t * self
    }
}

impl Div<f64> for Vec3{
    type Output = Vec3;

    fn div(self, t: f64) -> Self::Output{
        return (1.0/t) * self;
    }
}

pub type Color = Vec3;
#[allow(dead_code)]
pub type Point = Vec3;
