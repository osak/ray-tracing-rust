use std::f64::consts::PI;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3, // Horizontal vector from the left edge to the right edge
    vertical: Vec3,   // Vertical vector from the bottom edge to the top edge
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov * PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h; // y: [-tan(θ/2), tan(θ/2)]
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = up.cross(&w).unit_vector();
        let v = w.cross(&u).unit_vector();

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Self {
            origin: look_from,
            horizontal,
            vertical,
            lower_left_corner: look_from - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let x_comp = u * self.horizontal;
        let y_comp = v * self.vertical;
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + x_comp + y_comp - self.origin,
        }
    }
}
