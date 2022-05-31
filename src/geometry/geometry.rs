use super::HitRecord;

use crate::aabb::Aabb;
use crate::math::Ray;

pub trait Geometry {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool;
}
