use super::*;

use crate::random_unit_vector;

use crate::geometry::HitRecord;

use crate::math::*;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        return Lambertian {
            albedo,
        };
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal() + random_unit_vector();
        *scattered = Ray::new(rec.position(), scatter_direction, r_in.time());
        *attenuation = self.albedo;

        return true;
    }
}
