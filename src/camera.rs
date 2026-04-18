use indicatif::ProgressBar;

use crate::{
    geometry::{Color, Interval, Point3, Ray, Vec3},
    hittable::World,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Point3,
    pixel_delta_v: Point3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let img_height = if image_height < 1 { 1 } else { image_height };

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / img_height as f64);
        let center = Point3::new(0, 0, 0);

        let viewport_u = Vec3::new(viewport_width, 0, 0);
        let viewport_v = Vec3::new(0, -viewport_height, 0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / img_height as f64;

        let viewport_top_left =
            center - Vec3::new(0, 0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_top_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &World) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let bar = ProgressBar::new((self.image_height * self.image_width) as u64);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                self.get_ray_color(Ray::new(self.center, ray_direction), &world)
                    .write_ppm();
                bar.tick();
            }
        }
        bar.finish();
    }

    pub fn get_ray_color(&self, r: Ray, world: &World) -> Color {
        if let Some(rec) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
            return 0.5 * (rec.normal + Color::new(1, 1, 1));
        }
        let a = 0.5 * (r.direction.normalized().y + 1.0);
        (1.0 - a) * Color::new(1, 1, 1) + a * Color::new(0.5, 0.7, 1)
    }
}
