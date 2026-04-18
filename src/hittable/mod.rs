pub mod sphere;

use crate::geometry::{Interval, Point3, Ray, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64) -> Self {
        Self {
            p,
            normal: Vec3::zero(),
            t,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord>;
}

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for World {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(
                r,
                Interval::new(
                    t_interval.min,
                    if let Some(ref c) = closest {
                        c.t
                    } else {
                        t_interval.max
                    },
                ),
            ) {
                closest = Some(rec);
            }
        }

        closest
    }
}
