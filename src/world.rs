use crate::hittable::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::util::random;
use crate::vec3::Vec3;

use std::rc::Rc;

pub fn default_scene() -> HittableList {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian {
            albedo: Vec3::new(0.1, 0.2, 0.5),
        }),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        }),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Dielectric { ref_idx: 1.5 }),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Rc::new(Dielectric { ref_idx: 1.5 }),
    )));

    HittableList::new(list)
}

pub fn random_scene() -> HittableList {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
    )));

    for _ in 0..50 {
        let choose_mat = random();
        let center = Vec3::new(-5.0 + 10.0 * random(), 0.2, -5.0 + 10.0 * random());
        if (center.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
            if choose_mat < 0.6 {
                list.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Rc::new(Lambertian {
                        albedo: Vec3::new(
                            random() * random(),
                            random() * random(),
                            random() * random(),
                        ),
                    }),
                )));
            }
        } else if choose_mat < 0.8 {
            list.push(Box::new(Sphere::new(
                center,
                0.2,
                Rc::new(Metal::new(
                    Vec3::new(
                        0.5 * (1.0 + random()),
                        0.5 * (1.0 + random()),
                        0.5 * (1.0 + random()),
                    ),
                    0.5 * random(),
                )),
            )));
        } else {
            list.push(Box::new(Sphere::new(
                center,
                0.2,
                Rc::new(Dielectric { ref_idx: 1.5 }),
            )));
        }
    }

    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric { ref_idx: 1.5 }),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    HittableList::new(list)
}
