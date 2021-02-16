use std::cmp::Ordering;

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::object::Object;
use crate::material::Material;

trait ObjectLike : Hittable + Material {}

pub struct ObjectList<'a> {
    pub objects: Vec<Box<dyn Object + 'a>>
}

fn f64_cmp(x: f64, y: f64) -> Ordering {
    if x == y {
        Ordering::Equal
    } else if x < y {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

impl<'a> ObjectList<'a> {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(&Box<dyn Object + 'a>, HitRecord)> {
        self.objects.iter()
            .filter_map(|obj| obj.hit(ray, t_min, t_max).map(|hr| (obj, hr)))
            .min_by(|x, y| f64_cmp(x.1.t, y.1.t))
    }
}