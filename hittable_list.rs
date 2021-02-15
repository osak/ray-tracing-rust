use std::cmp::Ordering;

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::material::Material;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable<dyn Material>>>,
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

impl<'a, M: Material + 'a> Hittable<M> for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<M>> {
        self.objects.iter()
            .filter_map(|obj| obj.hit(ray, t_min, t_max))
            .min_by(|x, y| f64_cmp(x.t, y.t))
    }
}