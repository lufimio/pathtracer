use std::sync::Arc;

use crate::{
    geometry::{Interval, Point3, Ray},
    hittable::{HitRecord, Hittable, Object},
};

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_extrema(a: Point3, b: Point3) -> Self {
        Self {
            x: Interval::new(f64::min(a.x, b.x), f64::max(a.x, b.x)),
            y: Interval::new(f64::min(a.y, b.y), f64::max(a.y, b.y)),
            z: Interval::new(f64::min(a.z, b.z), f64::max(a.z, b.z)),
        }
    }

    pub fn containing(a: AABB, b: AABB) -> AABB {
        Self::new(
            Interval::containing(a.x, b.x),
            Interval::containing(a.y, b.y),
            Interval::containing(a.z, b.z),
        )
    }

    pub fn axis_interval(self, n: usize) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid Axis {}, expected 0-2", n),
        }
    }

    pub fn hit(self, r: Ray, t_interval: Interval) -> bool {
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / r.direction[axis];

            let t0 = (ax.min - r.origin[axis]) * adinv;
            let t1 = (ax.max - r.origin[axis]) * adinv;

            let mut ray_t = t_interval;
            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug, Clone)]
pub struct BVHNode {
    left: Arc<Object>,
    right: Arc<Object>,
    bbox: AABB,
}

impl Hittable for BVHNode {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_interval) {
            None
        } else {
            let left_rec = self.left.hit(r, t_interval);
            if let Some(right_rec) = self.right.hit(
                r,
                Interval::new(
                    t_interval.min,
                    match left_rec {
                        Some(ref rec) => rec.t,
                        None => t_interval.max,
                    },
                ),
            ) {
                Some(right_rec)
            } else {
                left_rec
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
