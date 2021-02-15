use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Vec3, outward_normal: &Vec3, t: f64) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal.clone(),
            false => -1.0 * outward_normal,
        };

        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}