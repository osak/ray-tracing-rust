use impl_ops::*;
use ops::Neg;
use rand::Rng;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::ops::{self, Range};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    v: __m256d,
}

impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    unsafe {
        Vec3 {
            v: _mm256_add_pd(lhs.v, rhs.v),
        }
    }
});

impl_op_ex!(-|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    unsafe {
        Vec3 {
            v: _mm256_sub_pd(lhs.v, rhs.v),
        }
    }
});

impl_op_ex!(*|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    unsafe {
        Vec3 {
            v: _mm256_mul_pd(lhs.v, rhs.v),
        }
    }
});

impl_op_ex_commutative!(*|lhs: &Vec3, rhs: f64| -> Vec3 {
    unsafe {
        Vec3 {
            v: _mm256_mul_pd(lhs.v, _mm256_set_pd(rhs, rhs, rhs, rhs)),
        }
    }
});

impl_op_ex!(/ |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    unsafe {
        Vec3 {
            v: _mm256_div_pd(lhs.v, rhs.v),
        }
    }
});

impl_op_ex!(/ |lhs: &Vec3, rhs: f64| -> Vec3 {
    unsafe {
        Vec3 {
            v: _mm256_div_pd(lhs.v, _mm256_set_pd(rhs, rhs, rhs, rhs)),
        }
    }
});

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        unsafe {
            Self {
                v: _mm256_set_pd(x, y, z, 0.0),
            }
        }
    }

    pub fn x(&self) -> f64 {
        let mut dst = [0.0; 4];
        unsafe {
            _mm256_store_pd(dst.as_mut_ptr(), self.v);
            dst[3]
        }
    }

    pub fn y(&self) -> f64 {
        let mut dst = [0.0; 4];
        unsafe {
            _mm256_store_pd(dst.as_mut_ptr(), self.v);
            dst[2]
        }
    }

    pub fn z(&self) -> f64 {
        let mut dst = [0.0; 4];
        unsafe {
            _mm256_store_pd(dst.as_mut_ptr(), self.v);
            dst[1]
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v = Self::new(
                rng.gen_range::<f64, Range<f64>>(-1.0..1.0),
                rng.gen_range::<f64, Range<f64>>(-1.0..1.0),
                rng.gen_range::<f64, Range<f64>>(-1.0..1.0),
            );
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        let v = self * rhs;
        let mut dst = [0.0; 4];

        unsafe {
            _mm256_store_pd(dst.as_mut_ptr(), v.v);
            dst[3] + dst[2] + dst[1]
        }
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        /*
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
        */

        let l = self.v;
        let r = rhs.v;
        unsafe {
            let l0 = _mm256_permute4x64_pd(l, 0b10011100);
            let r0 = _mm256_permute4x64_pd(r, 0b10011100);
            let c1 = _mm256_mul_pd(l, r0);
            let c2 = _mm256_mul_pd(l0, r);
            let c = _mm256_sub_pd(c1, c2);
            Self {
                v: _mm256_permute4x64_pd(c, 0b10011100),
            }
        }
    }

    pub fn reflect(&self, norm: &Self) -> Self {
        self - 2.0 * self.dot(norm) * norm
    }

    pub fn refract(&self, norm: &Self, refraction_ratio: f64) -> Self {
        let uv = self.unit_vector();
        let cos_theta = (-uv).dot(norm).min(1.0);
        let r_out_perp = refraction_ratio * (uv + cos_theta * norm);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * norm;

        r_out_perp + r_out_parallel
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn near_zero(&self) -> bool {
        let mut dst = [0.0; 4];

        unsafe {
            _mm256_store_pd(dst.as_mut_ptr(), self.v);
        }
        dst[3].abs() < 1e-8 && dst[2].abs() < 1e-8 && dst[1].abs() < 1e-8
    }
}

pub type Color = Vec3;
