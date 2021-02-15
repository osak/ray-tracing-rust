mod vec3;

use vec3::{Color};

fn print_ppm_header(w: i32, h: i32) {
    println!("P3");
    println!("{} {}", w, h);
    println!("{}", 255);
}

fn print_ppm_pixel(color: &Color) {
    let r = (color.x.sqrt() * 255.999) as i32;
    let g = (color.y.sqrt() * 255.999) as i32;
    let b = (color.z.sqrt() * 255.999) as i32;

    println!("{} {} {}", r, g, b)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let w = 400;
    let h = ((w as f64) / aspect_ratio) as i32;
    print_ppm_header(w, h);

    for j in (0..h).rev() {
        for i in 0..w {
            let pixel = Color {
                x: (i as f64) / ((w - 1) as f64),
                y: (j as f64) / ((h - 1) as f64),
                z: 0.25,
            };
            print_ppm_pixel(&pixel);
        }
    }
}