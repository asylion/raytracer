use std::ops;

#[derive(Debug, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        let e = self.e;
        (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        let e = self.e;
        e[0] * e[0] + e[1] * e[1] + e[2] * e[2]
    }

    pub fn make_unit_vector(&mut self) {
        let length = self.length();
        let k = 1.0 / length;

        *self *= k;
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    let x = v1[1] * v2[2] - v1[2] * v2[1];
    let y = v1[2] * v2[0] - v1[0] * v2[2];
    let z = v1[0] * v2[1] - v1[1] * v2[0];
    Vec3::new(x, y, z)
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(mut self, other: Vec3) -> Self {
        self[0] += other[0];
        self[1] += other[1];
        self[2] += other[2];
        self
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, other: Vec3) -> Self {
        self[0] -= other[0];
        self[1] -= other[1];
        self[2] -= other[2];
        self
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(mut self, other: Vec3) -> Self {
        self[0] *= other[0];
        self[1] *= other[1];
        self[2] *= other[2];
        self
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(mut self, num: f32) -> Self {
        self[0] *= num;
        self[1] *= num;
        self[2] *= num;
        self
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, mut vec: Vec3) -> Vec3 {
        vec[0] *= self;
        vec[1] *= self;
        vec[2] *= self;
        vec
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(mut self, other: Vec3) -> Self {
        self[0] /= other[0];
        self[1] /= other[1];
        self[2] /= other[2];
        self
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(mut self, num: f32) -> Self {
        self[0] /= num;
        self[1] /= num;
        self[2] /= num;
        self
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(mut self) -> Vec3 {
        self[0] = -self[0];
        self[1] = -self[1];
        self[2] = -self[2];
        self
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        &self.e[i]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        &mut self.e[i]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, num: f32) {
        self.e[0] *= num;
        self.e[1] *= num;
        self.e[2] *= num;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, num: f32) {
        self.e[0] /= num;
        self.e[1] /= num;
        self.e[2] /= num;
    }
}

pub fn unit_vector(vec: &Vec3) -> Vec3 {
    vec.clone() / vec.length()
}
