pub mod color;
pub mod ray;
pub mod vec3;

pub use color::*;
pub use ray::*;
pub use vec3::*;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * dot(v, n) * n;
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n);
    let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * n;

    return r_out_parallel + r_out_perp;
}

pub fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) *  (1.0 - cosine).powi(5);
}
