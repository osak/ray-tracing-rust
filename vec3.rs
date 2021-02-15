use impl_ops::*;
use rand::Rng;
use std::ops::{self, Range};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3 {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
        z: lhs.z + rhs.z,
    }
});

impl_op_ex!(- |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3 {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z,
    }
});

impl_op_ex!(* |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3 {
        x: lhs.x * rhs.x,
        y: lhs.y * rhs.y,
        z: lhs.z * rhs.z,
    }
});

impl_op_ex!(* |lhs: &Vec3, rhs: f64| -> Vec3 {
    Vec3 {
        x: lhs.x * rhs,
        y: lhs.y * rhs,
        z: lhs.z * rhs,
    }
});

impl_op_ex!(* |lhs: f64, rhs: &Vec3| -> Vec3 { rhs * lhs });

impl_op_ex!(/ |lhs: &Vec3, rhs: f64| -> Vec3 {
    Vec3 {
        x: lhs.x / rhs,
        y: lhs.y / rhs,
        z: lhs.z / rhs,
    }
});

impl_op_ex!(/ |lhs: f64, rhs: &Vec3| -> Vec3 { rhs / lhs });

impl Vec3 {
    pub fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Self {
                x: rng.gen_range::<f64, Range<f64>>(-1.0..1.0),
                y: rng.gen_range::<f64, Range<f64>>(-1.0..1.0),
                z: rng.gen_range::<f64, Range<f64>>(-1.0..1.0),
            };
            if p.length() <= 1.0 {
                return p;
            }
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
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
}

pub type Color = Vec3;