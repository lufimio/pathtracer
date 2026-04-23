use std::sync::Arc;

use crate::{
    geometry::{Color, Ray, near_zero, random_unit_vector},
    hittable::HitRecord,
    material::Scatter,
    texture::{Sample, Texture},
};

#[derive(Debug, Clone)]
pub struct Lambertian {
    tex: Arc<Texture>,
}

impl Lambertian {
    pub fn new(tex: Arc<Texture>) -> Self {
        Self { tex }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _r: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if near_zero(scatter_direction) {
            scatter_direction = rec.normal
        }

        Some((
            Ray::new(rec.p, scatter_direction),
            self.tex.sample(rec.u, rec.v, rec.p),
        ))
    }
}
