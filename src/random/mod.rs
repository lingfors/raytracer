use rand::Rng;

pub fn double_unit() -> f64 {
    return double_range(0.0, 1.0);
}

pub fn double_range(min: f64, max: f64) -> f64 {
    return rand::thread_rng().gen_range(min, max);
}
