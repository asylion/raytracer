use crate::vec3::Vec3;

use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random(), random(), random()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random(), random(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn random() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}
