mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;
mod world;

use crate::util::random;
use camera::Camera;
use hittable::Hittable;
use ray::Ray;
use vec3::unit_vector;
use vec3::Vec3;

use std::f32;

fn main() {
    ray_trace();
}

fn color<T: Hittable>(ray: &Ray, world: &T, depth: i32) -> Vec3 {
    if let Some(record) = world.hit(&ray, 0.001, f32::MAX) {
        if depth < 50 {
            if let Some(scatter) = record.material.scatter(ray, &record) {
                return scatter.attenuation * color(&scatter.ray, world, depth + 1);
            }
        }

        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn ray_trace() {
    let nx = 600;
    let ny = 400;
    let ns = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let world = world::default_scene();

    let lookfrom = Vec3::new(15.0, 3.0, 5.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
    );
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + random()) / nx as f32;
                let v = (j as f32 + random()) / ny as f32;
                let ray = camera.get_ray(u, v);
                col += color(&ray, &world, 0);
            }

            col /= ns as f32;

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
