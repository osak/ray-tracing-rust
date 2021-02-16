use rand::Rng;

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
        if scatter_dir.near_zero() {
            None
        } else {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered_ray: Ray {
                    origin: hit_record.p,
                    direction: scatter_dir,
                }
            })
        }
    }
}


pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray.direction.reflect(&hit_record.normal);
        if reflected.dot(&hit_record.normal) > 0.0 {
            let scatter_dir = reflected + self.fuzz * Vec3::random_in_unit_sphere();
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered_ray: Ray { origin: hit_record.p, direction: scatter_dir },
            })
        } else {
            None
        }
    }
}


pub struct Dielectric {
    pub refraction_index: f64,
}

fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
    let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
    r0 + (1.0-r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let cos_theta = (-ray.direction.unit_vector()).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflectance = reflectance(cos_theta, refraction_ratio);
        let mut rng = rand::thread_rng();

        let direction = if refraction_ratio * sin_theta > 1.0 || reflectance > rng.gen() {
            ray.direction.reflect(&hit_record.normal)
        } else {
            ray.direction.refract(&hit_record.normal, refraction_ratio)
        };

        Some(ScatterRecord {
            attenuation: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            scattered_ray: Ray { origin: hit_record.p, direction },
        })
    }
}