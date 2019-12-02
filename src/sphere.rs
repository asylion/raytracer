use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::dot;
use crate::vec3::Vec3;

use std::rc::Rc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin().clone() - self.center.clone();
        let a = dot(ray.direction(), ray.direction());
        let b = 2.0 * dot(&oc, ray.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let mut root = (-b - discriminant.sqrt()) / (2.0 * a);
            if root < t_max && root > t_min {
                return Some(get_hit_record(&self, ray, root));
            }

            root = (-b + discriminant.sqrt()) / (2.0 * a);
            if root < t_max && root > t_min {
                return Some(get_hit_record(&self, ray, root));
            }
        }

        None
    }
}

fn get_hit_record(sphere: &Sphere, ray: &Ray, root: f32) -> HitRecord {
    let p = ray.point_at_parameter(root);
    let normal = (p.clone() - sphere.center.clone()) / sphere.radius;

    HitRecord {
        t: root,
        p: p.clone(),
        normal: normal,
        material: Rc::clone(&sphere.material),
    }
}
