mod camera;
mod hit_record;
mod hittable;
mod material;
mod object;
mod object_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use material::{Dielectric, Lambertian, Metal};
use object_list::ObjectList;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Vec3};

fn ray_color(ray: &Ray, world: &ObjectList, depth: i32) -> Color {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some((obj, hr)) = world.hit(ray, 0.001, 1.0 / 0.0) {
        if let Some(sr) = obj.scatter(ray, &hr) {
            return sr.attenuation * ray_color(&sr.scattered_ray, world, depth - 1);
        }
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0); // Normalize to [0.0, 1.0]
    let white = Vec3::new(1.0, 1.0, 1.0);
    let sky_blue = Vec3::new(0.5, 0.7, 1.0);

    return (1.0 - t) * white + t * sky_blue;
}

fn print_ppm_header(w: i32, h: i32) {
    println!("P3");
    println!("{} {}", w, h);
    println!("{}", 255);
}

fn clamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

fn print_ppm_pixel(color: &Color) {
    let r = (clamp(color.x().sqrt(), 0.0, 0.999) * 256.0) as i32;
    let g = (clamp(color.y().sqrt(), 0.0, 0.999) * 256.0) as i32;
    let b = (clamp(color.z().sqrt(), 0.0, 0.999) * 256.0) as i32;

    println!("{} {} {}", r, g, b)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let w = 400;
    let h = ((w as f64) / aspect_ratio) as i32;

    let sphere1 = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let sphere2 = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };
    let sphere_left = Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
    };
    let sphere_right = Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
    };
    let material_ground = Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    };
    let material = Lambertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    };
    let material_left = Dielectric {
        refraction_index: 1.5,
    };
    let material_right = Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    let world = ObjectList {
        objects: vec![
            Box::new(object::new(&sphere1, &material)),
            Box::new(object::new(&sphere2, &material_ground)),
            Box::new(object::new(&sphere_left, &material_left)),
            Box::new(object::new(&sphere_right, &material_right)),
        ],
    };

    let look_from = Vec3::new(-2.0, 2.0, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(look_from, look_at, up, 20.0, aspect_ratio);

    print_ppm_header(w, h);

    let mut rng = rand::thread_rng();
    for j in (0..h).rev() {
        eprint!("\rScanlines remaining: {}  ", j);
        for i in 0..w {
            let color = (0..100)
                .map(|_| {
                    let u = (i as f64 + rng.gen::<f64>()) / ((w - 1) as f64);
                    let v = (j as f64 + rng.gen::<f64>()) / ((h - 1) as f64);
                    let ray = camera.get_ray(u, v);
                    ray_color(&ray, &world, 50)
                })
                .fold(Vec3::new(0.0, 0.0, 0.0), |a, b| a + b)
                / 100.0;
            print_ppm_pixel(&color);
        }
    }

    eprintln!("Done");
}
