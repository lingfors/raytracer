use super::*;

use crate::unit_vector;

use crate::geometry::HitRecord;

use crate::math::*;

use crate::random;

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        return Dielectric{ refraction_index };
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat: f64;

        if rec.front_face() {
            etai_over_etat = 1.0 / self.refraction_index;
        } else {
            etai_over_etat = self.refraction_index;
        }

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = f64::min(dot(-unit_direction, rec.normal()), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if (etai_over_etat * sin_theta) > 1.0 {
            let reflected = reflect(unit_direction, rec.normal());
            *scattered = Ray::new(rec.position(), reflected, r_in.time());
            return true;
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random::double_unit() < reflect_prob {
            let reflected = reflect(unit_direction, rec.normal());
            *scattered = Ray::new(rec.position(), reflected, r_in.time());
            return true;
        }

        let refracted = refract(unit_direction, rec.normal(), etai_over_etat);
        *scattered = Ray::new(rec.position(), refracted, r_in.time());

        return true;
    }
}
