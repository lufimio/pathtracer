use crate::{
    geometry::{Interval, Point3, Ray, Vec3},
    hittable::{HitRecord, Hittable, bvh::AABB},
    material::Material,
};
use core::f32;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Arc<Material>,
    bbox: AABB,
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return Option::None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !t_interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t_interval.surrounds(root) {
                return Option::None;
            }
        }

        let mut rec = HitRecord::new(r.at(root), Arc::clone(&self.mat), root);
        let outward_normal = (rec.p - self.center) / self.radius;
        let theta = f32::acos(-outward_normal.y);
        let phi = f32::atan2(-outward_normal.z, outward_normal.x) + f32::consts::PI;
        rec.set_uv_coords(phi / (2.0 * f32::consts::PI), theta / f32::consts::PI);
        rec.set_face_normal(r, outward_normal);

        Option::Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, mat: Arc<Material>) -> Self {
        let radius = radius.max(0.0);
        Self {
            center,
            radius,
            mat,
            bbox: AABB::from_extrema(center - Vec3::ONE * radius, center + Vec3::ONE * radius),
        }
    }
}
