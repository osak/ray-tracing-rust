use impl_ops::*;
use rand::Rng;
use std::ops::{self, Neg, Range};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3 {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
        z: lhs.z + rhs.z,
    }
});

impl_op_ex!(-|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3 {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z,
    }
});

impl_op_ex!(*|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3 {
        x: lhs.x * rhs.x,
        y: lhs.y * rhs.y,
        z: lhs.z * rhs.z,
    }
});

impl_op_ex!(*|lhs: &Vec3, rhs: f64| -> Vec3 {
    Vec3 {
        x: lhs.x * rhs,
        y: lhs.y * rhs,
        z: lhs.z * rhs,
    }
});

impl_op_ex!(*|lhs: f64, rhs: &Vec3| -> Vec3 { rhs * lhs });

impl_op_ex!(/ |lhs: &Vec3, rhs: f64| -> Vec3 {
    Vec3 {
        x: lhs.x / rhs,
        y: lhs.y / rhs,
        z: lhs.z / rhs,
    }
});

impl_op_ex!(/ |lhs: f64, rhs: &Vec3| -> Vec3 { rhs / lhs });

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v = Self {
                x: rng.gen_range::<f64, Range<f64>>(-1.0..1.0),
                y: rng.gen_range::<f64, Range<f64>>(-1.0..1.0),
                z: rng.gen_range::<f64, Range<f64>>(-1.0..1.0),
            };
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
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
        self.x.abs() < 1e-8 && self.y.abs() < 1e-8 && self.z.abs() < 1e-8
    }
}

pub type Color = Vec3;
