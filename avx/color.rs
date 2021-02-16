use crate::vec3::Vec3;

pub fn to_ppm_color(color: &Vec3) -> (i32, i32, i32) {
    let ppm: Vec3 = color.sqrt().clamp(0.0, 0.999) * 256.0;
    let vals: (f64, f64, f64) = ppm.into();
    (vals.0 as i32, vals.1 as i32, vals.2 as i32)
}