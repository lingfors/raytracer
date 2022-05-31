use crate::math::*;

pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Aabb {
        return Aabb {
            min,
            max,
        };
    }

    fn new_default() -> Aabb {
        return Aabb {
            min: Vec3::new_default(),
            max: Vec3::new_default(),
        };
    }

    fn intersect(&self, r: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.min[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };

            if tmax <= tmin {
                return false;
            }
        }

        return true;
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let min = Vec3::new(
        box0.min.x().min(box1.min.x()),
        box0.min.y().min(box1.min.y()),
        box0.min.z().min(box1.min.z()),
    );

    let max = Vec3::new(
        box0.max.x().max(box1.max.x()),
        box0.max.y().max(box1.max.y()),
        box0.max.z().max(box1.max.z()),
    );

    return Aabb::new(min, max);
}
