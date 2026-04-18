mod camera;
mod geometry;
mod hittable;

use crate::{
    camera::Camera,
    geometry::Point3,
    hittable::{World, sphere::Sphere},
};

fn main() {
    let mut world = World::empty();
    world.add(Sphere::new(Point3::new(0, -100.5, -1), 100));
    world.add(Sphere::new(Point3::new(0, 0, -1), 0.5));

    let camera = Camera::new(16. / 9., 400);
    camera.render(&world);
}
