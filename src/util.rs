
// Constants
const INFINITY: f64 = f64::INFINITY;
const PI: f64 = std::f64::consts::PI;

// Util Functions
pub fn degs_to_rads(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn rads_to_degs(rad: f64) -> f64 {
    rad * 180.0 / PI
}