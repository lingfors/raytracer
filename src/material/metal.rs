use super::*;

use crate::random_in_unit_sphere;

use crate::geometry::HitRecord;

use crate::math::*;

pub struct Metal {
    albedo: Color,
    fuzzing: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzzing: f64) -> Metal {
        return Metal{ albedo, fuzzing };
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal());
        *scattered = Ray::new(rec.position(), reflected + self.fuzzing * random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo;

        return dot(scattered.direction(), rec.normal()) > 0.0;
    }
}
