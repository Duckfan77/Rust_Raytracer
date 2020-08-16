
// Constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Util Functions
pub fn degs_to_rads(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn rads_to_degs(rad: f64) -> f64 {
    rad * 180.0 / PI
}

use rand::Rng;

pub fn random_double() -> f64 {
    // Gets a random double in [0,1)
    rand::thread_rng().gen_range(0.0, 1.0)
}

// Gets a random double in [min,max). panics if min >= max
pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min, max)
}

pub fn random_int_range(min: u32, max: u32) -> u32 {
    random_double_range(min as f64, max as f64 + 1.0) as u32
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min
    }
    if x > max {
        return max
    }
    return x
}