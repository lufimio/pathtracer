pub mod dielectric;
pub mod lambertian;
pub mod metal;

use enum_dispatch::enum_dispatch;
use crate::{
    geometry::{Color, Ray},
    hittable::HitRecord,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
};


#[enum_dispatch]
pub trait Scatter {
    fn scatter(&self, _r: Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        Option::None
    }
}

#[enum_dispatch(Scatter)]
#[derive(Debug)]
pub enum Material {
    Lambertian,
    Metal,
    Dielectric,
}
