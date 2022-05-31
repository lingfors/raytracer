use crate::math::*;

pub struct HitRecord {
    position: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(position: Vec3, normal: Vec3, t: f64, front_face: bool) -> HitRecord {
        return HitRecord {
            position,
            normal,
            t,
            front_face,
        };
    }

    pub fn new_default() -> HitRecord {
        return HitRecord {
            position: Vec3::new_default(),
            normal: Vec3::new_default(),
            t: 0.0,
            front_face: false,
        };
    }

    pub fn position(&self) -> Vec3 {
        return self.position;
    }

    pub fn normal(&self) -> Vec3 {
        return self.normal;
    }

    pub fn t(&self) -> f64 {
        return self.t;
    }

    pub fn front_face(&self) -> bool {
        return self.front_face;
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}
