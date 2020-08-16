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
        let mut u = p.x() - f64::floor(p.x());
        let mut v = p.y() - f64::floor(p.y());
        let mut w = p.z() - f64::floor(p.z());
        u = u*u*(3.0 - 2.0*u);
        v = v*v*(3.0 - 2.0*v);
        w = w*w*(3.0 - 2.0*w);

        let i = f64::floor(p.x());
        let j = f64::floor(p.y());
        let k = f64::floor(p.z());

        let mut c = [[[0f64; 2]; 2]; 2];

        for di in 0..2i32 {
            for dj in 0..2i32 {
                for dk in 0..2i32 {
                    c[di as usize][dj as usize][dk as usize] = self.ranfloat[(
                        self.perm_x[((i as i32 + di) & 255) as usize] ^
                        self.perm_y[((j as i32 + dj) & 255) as usize] ^
                        self.perm_z[((k as i32 + dk) & 255) as usize]) as usize
                    ]
                }
            }
        }

        return trilinear_interp(c, u, v, w)
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

fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w:f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2{
                accum += (i as f64 *u + (1.0-i as f64)*(1.0-u))*
                         (j as f64 *v + (1.0-j as f64)*(1.0-v))*
                         (k as f64 *w + (1.0-k as f64)*(1.0-w))*c[i as usize][j as usize][k as usize];
            }
        }
    }

    return accum
}
