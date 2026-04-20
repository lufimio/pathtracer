use rand::random;

use crate::{
    geometry::{Color, Ray, Vec3},
    hittable::HitRecord,
    material::Scatterable,
};

pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * normal
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(Vec3::dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, ri: f64) -> f64 {
    let r0 = (1.0 - ri) / (1.0 + ri);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let ri = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = r.direction.normalized();
        let cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let direction = if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > random() {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, ri)
        };

        Some((Ray::new(rec.p, direction), Color::one()))
    }
}
