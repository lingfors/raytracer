use super::*;

use crate::aabb::*;
use crate::math::*;

pub struct Sphere {
    center0: Vec3,
    center1: Vec3,
    radius: f64,
    time0: f64,
    time1: f64,
}

impl Sphere {
    pub fn new(center0: Vec3, center1: Vec3, radius: f64, time0: f64, time1: f64) -> Sphere {
        return Sphere{ center0, center1, radius, time0, time1 };
    }

    pub fn new_stationary(center: Vec3, radius: f64) -> Sphere {
        return Sphere{ center0: center, center1: center, radius, time0: 0.0, time1: 1.0 };
    }

    pub fn center(&self, time: f64) -> Vec3 {
        return self.center0 + (((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0));
    }
}

impl Geometry for Sphere {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
    
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp1 = (-half_b - root) / a;

            if (temp1 < t_max) && (temp1 > t_min) {
                let position = r.at(temp1);
                let normal = (position - self.center(r.time())) / self.radius;

                *rec = HitRecord::new(
                    position,
                    normal,
                    temp1,
                    false
                );

                let outward_normal = (rec.position() - self.center(r.time())) / self.radius;
                rec.set_face_normal(r, outward_normal);

                return true;
            }

            let temp2 = (-half_b + root) / a;

            if (temp2 < t_max) && (temp2 > t_min) {
                let position = r.at(temp2);
                let normal = (position - self.center(r.time())) / self.radius;

                *rec = HitRecord::new(
                    position,
                    normal,
                    temp2,
                    false
                );

                let outward_normal = (rec.position() - self.center(r.time())) / self.radius;
                rec.set_face_normal(r, outward_normal);

                return true;
            }
        }

        return false;
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool {
        let v = Vec3::new(self.radius, self.radius, self.radius);

        let center0 = self.center(t0);
        let box0 = Aabb::new(center0 - v, center0 + v);

        let center1 = self.center(t1);
        let box1 = Aabb::new(center1 - v, center1 + v);

        *output_box = surrounding_box(&box0, &box1);

        return true;
    }
}
