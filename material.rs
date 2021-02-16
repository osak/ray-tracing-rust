use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit_record::HitRecord;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered_ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}


// Lambertian material
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let scatter_dir = hit_record.normal + Vec3::random_in_unit_sphere().unit_vector();
        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered_ray: Ray {
                origin: hit_record.p,
                direction: scatter_dir,
            }
        })
    }
}


pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let scatter_dir = ray.direction.reflect(&hit_record.normal);
        if scatter_dir.dot(&hit_record.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered_ray: Ray { origin: hit_record.p, direction: scatter_dir }
            })
        } else {
            None
        }
    }
}