use crate::{util::*, vec3, vec3::*};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranvec = [Vec3::new_e(); POINT_COUNT];
        for vec in ranvec.iter_mut().take(POINT_COUNT) {
            *vec = unit_vector(vec3::random_range(-1.0, 1.0));
        }

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();

        Perlin {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point) -> f64 {
        let u = p.x() - f64::floor(p.x());
        let v = p.y() - f64::floor(p.y());
        let w = p.z() - f64::floor(p.z());

        let i = f64::floor(p.x());
        let j = f64::floor(p.y());
        let k = f64::floor(p.z());

        let mut c = [[[Vec3::new_e(); 2]; 2]; 2];

        for di in 0..2i32 {
            for dj in 0..2i32 {
                for dk in 0..2i32 {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[(self.perm_x
                        [((i as i32 + di) & 255) as usize]
                        ^ self.perm_y[((j as i32 + dj) & 255) as usize]
                        ^ self.perm_z[((k as i32 + dk) & 255) as usize])
                        as usize]
                }
            }
        }

        perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        f64::abs(accum)
    }
}

fn perlin_generate_perm() -> [i32; POINT_COUNT] {
    let mut p = [0i32; POINT_COUNT];

    for (i, pi) in p.iter_mut().enumerate().take(POINT_COUNT) {
        *pi = i as i32;
    }

    permute(&mut p, POINT_COUNT as usize);

    p
}

fn permute(p: &mut [i32; POINT_COUNT], n: usize) {
    for i in (1..n).rev() {
        let target = random_int_range(0, i as i32) as usize;
        p.swap(i, target);
    }
}

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for (i, ci) in c.iter().enumerate() {
        for (j, cij) in ci.iter().enumerate() {
            for (k, cijk) in cij.iter().enumerate() {
                let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                    * dot(*cijk, weight_v);
            }
        }
    }

    accum
}

fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                    * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                    * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                    * c[i as usize][j as usize][k as usize];
            }
        }
    }

    accum
}
