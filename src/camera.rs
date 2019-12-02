use crate::ray::Ray;
use crate::vec3::Vec3;

use std::f32::consts::PI;

use crate::vec3::{cross, unit_vector};

use crate::util::random_in_unit_disk;

pub struct Camera {
    origin: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = unit_vector(&(lookfrom.clone() - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        Camera {
            origin: lookfrom.clone(),
            vertical: 2.0 * half_height * focus_dist * v.clone(),
            horizontal: 2.0 * half_width * focus_dist * u.clone(),
            lower_left_corner: lookfrom.clone()
                - half_width * focus_dist * u.clone()
                - half_height * focus_dist * v.clone()
                - focus_dist * w.clone(),
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u.clone() * rd.x() + self.v.clone() * rd.y();
        let direction = self.lower_left_corner.clone()
            + u * self.horizontal.clone()
            + v * self.vertical.clone()
            - self.origin.clone()
            - offset.clone();
        Ray::new(self.origin.clone() + offset, direction)
    }
}
