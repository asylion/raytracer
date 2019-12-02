use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util::{random, random_in_unit_sphere};
use crate::vec3::Vec3;
use crate::vec3::{dot, unit_vector};

use std::f32;

fn reflect(vec: &Vec3, n: &Vec3) -> Vec3 {
    return vec.clone() - 2.0 * dot(vec, n) * n.clone();
}

fn refract(vec: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = unit_vector(vec);
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refraction = ni_over_nt * (uv - n.clone() * dt) - n.clone() * discriminant.sqrt();
        Some(refraction)
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let target = rec.p.clone() + rec.normal.clone() + random_in_unit_sphere();

        Some(Scatter {
            ray: Ray::new(rec.p.clone(), target - rec.p.clone()),
            attenuation: self.albedo.clone(),
        })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(&unit_vector(ray_in.direction()), &rec.normal);

        Some(Scatter {
            ray: Ray::new(
                rec.p.clone(),
                reflected + self.fuzz * random_in_unit_sphere(),
            ),
            attenuation: self.albedo.clone(),
        })
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let outward_normal;
        let reflected = reflect(ray_in.direction(), &rec.normal);
        let mut refracted = Vec3::new(0.0, 0.0, 0.0);
        let ni_over_nt;
        let scattered;

        let reflect_prob;
        let cosine;

        if dot(ray_in.direction(), &rec.normal) > 0.0 {
            outward_normal = -rec.normal.clone();
            ni_over_nt = self.ref_idx;
            cosine =
                self.ref_idx * dot(ray_in.direction(), &rec.normal) / ray_in.direction().length();
        } else {
            outward_normal = rec.normal.clone();
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dot(ray_in.direction(), &rec.normal) / ray_in.direction().length();
        }

        if let Some(refraction) = refract(ray_in.direction(), &outward_normal, ni_over_nt) {
            refracted = refraction;
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }

        if random() < reflect_prob {
            scattered = Ray::new(rec.p.clone(), reflected);
        } else {
            scattered = Ray::new(rec.p.clone(), refracted);
        }

        Some(Scatter {
            ray: scattered,
            attenuation: Vec3::new(1.0, 1.0, 1.0),
        })
    }
}
