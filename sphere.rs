use crate::{hittable::Hittable, vec3::Vec3};
use crate::ray::Ray;
use crate::hit_record::HitRecord;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Solve a quadratic equation to find a real number `t` where
        // ray.origin() + t*ray.direction() is on this sphere.
        let v = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = v.dot(&ray.direction);
        let c = v.length_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 {
            return Option::None;
        }
        let sqrtd = discriminant.sqrt();

        // Root of the equation. Initialize with the first root.
        let mut root = (-half_b - sqrtd) / a;

        // If the first root is not within the expected range, try the other one.
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
        }

        // If neither root works, the ray is not considered hitting this sphere.
        if root < t_min || t_max < root {
            return Option::None;
        }

        let hit_point = ray.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;
        return Some(HitRecord::new(ray, hit_point, &outward_normal, root))
    }
}