use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::rc::Rc;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_record = None;
        let mut closest = t_max;

        for hittable in &self.list {
            if let Some(record) = hittable.hit(&ray, t_min, closest) {
                closest = record.t;
                closest_record = Some(record);
            }
        }

        closest_record
    }
}
