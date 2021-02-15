mod vec3;
mod ray;
mod camera;
mod hit_record;
mod hittable;
mod sphere;
mod hittable_list;

use camera::Camera;
use rand::Rng;
use vec3::{Vec3, Color};
use ray::Ray;
use hittable::Hittable;
use sphere::Sphere;
use hittable_list::HittableList;

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    if let Some(hr) = world.hit(ray, 0.0, 1.0/0.0) {
        return 0.5 * (hr.normal + Color { x: 1.0, y: 1.0, z: 1.0 });
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0); // Normalize to [0.0, 1.0]
    let white = Color { x: 1.0, y: 1.0, z: 1.0 };
    let sky_blue = Color { x: 0.5, y: 0.7, z: 1.0 };

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
    let r = (clamp(color.x.sqrt(), 0.0, 0.999) * 256.0) as i32;
    let g = (clamp(color.y.sqrt(), 0.0, 0.999) * 256.0) as i32;
    let b = (clamp(color.z.sqrt(), 0.0, 0.999) * 256.0) as i32;

    println!("{} {} {}", r, g, b)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let w = 400;
    let h = ((w as f64) / aspect_ratio) as i32;

    let world = HittableList {
        objects: vec![
            Box::new(Sphere { center: Vec3 { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5 }),
            Box::new(Sphere { center: Vec3 { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0 }),
        ]
    };

    let look_from = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let look_at = Vec3 { x: 0.0, y: 0.0, z: -1.0 };
    let up = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    let camera = Camera::new(look_from, look_at, up, 90.0, aspect_ratio);

    print_ppm_header(w, h);

    let mut rng = rand::thread_rng();
    for j in (0..h).rev() {
        eprint!("\rScanlines remaining: {}  ", j);
        for i in 0..w {
            let color = (0..100).map(|_| {
                let u = (i as f64 + rng.gen::<f64>()) / ((w - 1) as f64);
                let v = (j as f64 + rng.gen::<f64>()) / ((h - 1) as f64);
                let ray = camera.get_ray(u, v);
                ray_color(&ray, &world)
            })
            .fold(Color { x: 0.0, y: 0.0, z: 0.0 }, |a, b| a + b) / 100.0;
            print_ppm_pixel(&color);
        }
    }

    eprintln!("Done");
}