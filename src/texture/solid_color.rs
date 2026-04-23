use crate::{
    geometry::{Color, Point3},
    texture::Sample,
};

#[derive(Debug, Clone, Copy)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Sample for SolidColor {
    fn sample(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        self.albedo
    }
}
