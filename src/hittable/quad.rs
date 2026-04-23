use std::sync::Arc;

use glam::Vec3;

use crate::{
    geometry::{Interval, Point3, Ray},
    hittable::{HitRecord, Hittable, bvh::AABB},
    material::Material,
};

#[derive(Debug, Clone)]
pub struct Quad {
    corner: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<Material>,
    bbox: AABB,
    normal: Vec3,
    d: f32,
}

impl Quad {
    pub fn new(corner: Point3, u: Vec3, v: Vec3, mat: Arc<Material>) -> Self {
        let bbox_diagonal_1 = AABB::from_extrema(corner, corner + u + v);
        let bbox_diagonal_2 = AABB::from_extrema(corner + u, corner + v);
        let bbox = AABB::containing(bbox_diagonal_1, bbox_diagonal_2);
        let n = Vec3::cross(u, v);
        let normal = n.normalize();
        let d = normal.dot(corner);
        let w = n / Vec3::dot(n, n);

        Self {
            corner,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            d,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(r.direction);
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(r.origin)) / denom;
        if !t_interval.contains(t) {
            return None;
        }

        let intersection = r.at(t);
        let planar_hit_vector = intersection - self.corner;
        let alpha = self.w.dot(planar_hit_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_vector));

        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }

        let mut rec = HitRecord::new(intersection, Arc::clone(&self.mat), t);
        rec.set_face_normal(r, self.normal);
        Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
