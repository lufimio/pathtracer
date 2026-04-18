mod geometry;
mod hittable;

use std::f64;

use indicatif::ProgressBar;

use crate::{
    geometry::{Color, Interval, Point3, Ray, Vec3},
    hittable::{Hittable, World, sphere::Sphere},
};

fn get_ray_color(r: Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
        return 0.5 * (rec.normal + Color::new(1, 1, 1));
    }
    let a = 0.5 * (r.direction.normalized().y + 1.0);
    (1.0 - a) * Color::new(1, 1, 1) + a * Color::new(0.5, 0.7, 1)
}

fn main() {
    let ideal_aspect_ratio = 16.0 / 9.0;

    let img_width = 600;
    let img_height = (img_width as f64 / ideal_aspect_ratio) as u32;
    let img_height = if img_height < 1 { 1 } else { img_height };

    let mut world = World::new(vec![Box::new(Sphere::new(Point3::new(0, -100.5, -1), 100))]);
    world.add(Sphere::new(Point3::new(0, 0, -1), 0.5));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
    let camera_origin = Point3::new(0, 0, 0);

    let viewport_u = Vec3::new(viewport_width, 0, 0);
    let viewport_v = Vec3::new(0, -viewport_height, 0);

    let pixel_delta_u = viewport_u / img_width as f64;
    let pixel_delta_v = viewport_v / img_height as f64;

    let viewport_top_left =
        camera_origin - Vec3::new(0, 0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_location = viewport_top_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    print!("P3\n{} {}\n255\n", img_width, img_height);

    let bar = ProgressBar::new((img_height * img_width) as u64);
    for j in 0..img_height {
        for i in 0..img_width {
            let pixel_center =
                pixel00_location + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_origin;
            get_ray_color(Ray::new(camera_origin, ray_direction), &world).write_ppm();
            bar.tick();
        }
    }
    bar.finish();
}
