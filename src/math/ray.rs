use super::*;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Ray {
        return Ray {
            origin,
            direction,
            time
        };
    }

    pub fn new_default() -> Ray {
        return Ray {
            origin: Vec3::new_default(),
            direction: Vec3::new_default(),
            time: 0.0,
        };
    }

    pub fn origin(&self) -> Vec3  {
        return self.origin;
    }

    pub fn direction(&self) -> Vec3 {
        return self.direction;
    }

    pub fn time(&self) -> f64 {
        return self.time;
    }

    pub fn at(&self, t: f64) -> Vec3 {
        return self.origin + t * self.direction;
    }
}
