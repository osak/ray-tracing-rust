use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::hit_record::HitRecord;

pub trait Object : Hittable + Material {}

pub struct ObjectImpl<'a, H: Hittable + 'a, M: Material + 'a> {
    pub hittable: &'a H,
    pub material: &'a M,
}

impl<'a, H: Hittable, M: Material> Hittable for ObjectImpl<'a, H, M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.hittable.hit(ray, t_min, t_max)
    }
}

impl <'a, H: Hittable, M: Material> Material for ObjectImpl<'a, H, M> {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<crate::material::ScatterRecord> {
        self.material.scatter(ray, hit_record)
    }
}

impl <'a, H: Hittable, M: Material> Object for ObjectImpl<'a, H, M> {}

pub fn new<'a, H: Hittable, M: Material>(hittable: &'a H, material: &'a M) -> ObjectImpl<'a, H, M> {
    ObjectImpl {
        hittable,
        material,
    }
}