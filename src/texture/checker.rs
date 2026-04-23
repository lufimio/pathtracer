use std::sync::Arc;

use crate::{
    geometry::{Color, Point3},
    texture::{Sample, Texture},
};

#[derive(Debug, Clone)]
pub struct Checker {
    inv_scale: f32,
    even: Arc<Texture>,
    odd: Arc<Texture>,
}

impl Checker {
    pub fn new(scale: f32, even: Arc<Texture>, odd: Arc<Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl Sample for Checker {
    fn sample(&self, u: f32, v: f32, p: Point3) -> Color {
        let x = (self.inv_scale * p.x).floor();
        let y = (self.inv_scale * p.y).floor();
        let z = (self.inv_scale * p.z).floor();

        if (x + y + z) % 2.0 == 0.0 {
            self.even.sample(u, v, p)
        } else {
            self.odd.sample(u, v, p)
        }
    }
}
