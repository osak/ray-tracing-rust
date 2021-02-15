mod vec3;
mod ray;
mod camera;

use camera::Camera;
use vec3::{Vec3, Color};
use ray::Ray;

fn ray_color(ray: &Ray) -> Color {
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

    let look_from = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let look_at = Vec3 { x: 0.0, y: 0.0, z: -1.0 };
    let up = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    let camera = Camera::new(look_from, look_at, up, 90.0, aspect_ratio);

    print_ppm_header(w, h);

    for j in (0..h).rev() {
        for i in 0..w {
            let u =(i as f64) / ((w - 1) as f64);
            let v = (j as f64) / ((h - 1) as f64);
            let ray = camera.get_ray(u, v);
            let color = ray_color(&ray);
            print_ppm_pixel(&color);
        }
    }
}