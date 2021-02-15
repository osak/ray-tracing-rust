use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::material::Material;

pub trait Hittable<M: Material> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<M>>;
}