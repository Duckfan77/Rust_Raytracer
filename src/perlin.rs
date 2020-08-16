use crate::{
    util::*,
    vec3::*,
};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranfloat = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            ranfloat[i] = random_double();
        }

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();

        Perlin {
            ranfloat: ranfloat,
            perm_x: perm_x,
            perm_y: perm_y,
            perm_z: perm_z,
        }
    }

    pub fn noise(&self, p: &Point) -> f64 {
        let _u = p.x() - f64::floor(p.x());
        let _v = p.y() - f64::floor(p.y());
        let _w = p.z() - f64::floor(p.z());

        let i = ((4.0*p.x()) as i32 & 255) as usize;
        let j = ((4.0*p.y()) as i32 & 255) as usize;
        let k = ((4.0*p.z()) as i32 & 255) as usize;

        return self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }
}

fn perlin_generate_perm() -> [i32; POINT_COUNT]{
    let mut p = [0i32; POINT_COUNT];

    for i in 0..POINT_COUNT {
        p[i] = i as i32;
    }

    permute(&mut p, POINT_COUNT as usize);

    return p
}

fn permute(p: &mut [i32; POINT_COUNT], n: usize) {
    for i in (1..n).rev() {
        let target = random_int_range(0, i as i32) as usize;
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}