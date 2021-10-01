use std::ops;

use rand::distributions::uniform::SampleRange;
use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn ones() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn is_near_zero(&self) -> bool {
        const THRESHOLD: f64 = 1e-8;
        self.x.abs() < THRESHOLD && self.y.abs() < THRESHOLD && self.z.abs() < THRESHOLD
    }

    pub fn as_rgb_vec(&self) -> [u8; 3] {
        [
            (self.x.sqrt() * 256.0).clamp(0.0, 255.0) as u8,
            (self.y.sqrt() * 256.0).clamp(0.0, 255.0) as u8,
            (self.z.sqrt() * 256.0).clamp(0.0, 255.0) as u8,
        ]
    }

    pub fn dot(lhs: &Self, rhs: &Self) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: &Self, rhs: &Self) -> Self {
        Self {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    pub fn normalize(&self) -> Self {
        *self / self.len()
    }

    pub fn rand() -> Self {
        Self {
            x: rand::thread_rng().gen(),
            y: rand::thread_rng().gen(),
            z: rand::thread_rng().gen(),
        }
    }

    pub fn rand_range<R: SampleRange<f64> + Clone>(range: R) -> Self {
        Self {
            x: rand::thread_rng().gen_range(range.clone()),
            y: rand::thread_rng().gen_range(range.clone()),
            z: rand::thread_rng().gen_range(range.clone()),
        }
    }

    pub fn rand_in_unit_sphere() -> Self {
        loop {
            let vec = Self::rand_range(-1.0..=1.0);
            if vec.len_squared() < 1.0 {
                break vec;
            }
        }
    }

    pub fn rand_on_unit_sphere() -> Self {
        Self::rand_in_unit_sphere().normalize()
    }

    pub fn rand_in_unit_disk() -> Self {
        loop {
            let vec = Self {
                x: rand::thread_rng().gen_range(-1.0..=1.0),
                y: rand::thread_rng().gen_range(-1.0..=1.0),
                z: 0.0,
            };

            if vec.len_squared() < 1.0 {
                break vec;
            }
        }
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        let scaled_normal = *normal * -Vec3::dot(self, normal);
        *self + 2.0 * scaled_normal
    }

    pub fn refract(&self, normal: &Vec3, ir_ratio: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&-*self, normal).min(1.0);
        let r_perp = ir_ratio * (*self + cos_theta * *normal);
        let r_para = -f64::sqrt(f64::abs(1.0 - r_perp.len_squared())) * *normal;
        r_perp + r_para
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output { x: rhs * self.x, y: rhs * self.y, z: rhs * self.z }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn vec3() {
        use super::Vec3;

        assert_eq!(-Vec3::new(1.0, 1.0, 1.0), Vec3::new(-1.0, -1.0, -1.0));

        assert_eq!(Vec3::new(1.0, 10.0, 100.0) + Vec3::new(2.0, 20.0, 200.0), Vec3::new(3.0, 30.0, 300.0));
        assert_eq!(Vec3::new(5.0, 50.0, 500.0) - Vec3::new(1.0, 10.0, 100.0), Vec3::new(4.0, 40.0, 400.0));
        assert_eq!(Vec3::new(5.0, 2.0, 3.0) * Vec3::new(4.0, 2.0, 2.0), Vec3::new(20.0, 4.0, 6.0));

        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 10.0, Vec3::new(10.0, 20.0, 30.0));
        assert_eq!(10.0 * Vec3::new(1.0, 2.0, 3.0), Vec3::new(10.0, 20.0, 30.0));
        assert_eq!(Vec3::new(10.0, 20.0, 30.0) / 10.0, Vec3::new(1.0, 2.0, 3.0));

        assert_eq!(Vec3::new(2.0, 3.0, 4.0).len_squared(), 29.0);
        assert_eq!(Vec3::new(2.0, 3.0, 4.0).len(), 29.0_f64.sqrt());

        let mut vec = Vec3::default();

        vec += Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec, Vec3::new(1.0, 2.0, 3.0));

        vec -= Vec3::new(0.0, 1.0, 1.0);
        assert_eq!(vec, Vec3::new(1.0, 1.0, 2.0));

        vec *= 6.0;
        assert_eq!(vec, Vec3::new(6.0, 6.0, 12.0));

        vec /= 3.0;
        assert_eq!(vec, Vec3::new(2.0, 2.0, 4.0));

        assert_eq!(Vec3::dot(
            &Vec3::new(2.0, 4.0, 3.0),
            &Vec3::new(6.0, 5.0, 1.0),
        ), 12.0 + 20.0 + 3.0);
        assert_eq!(Vec3::cross(
            &Vec3::new(1.0, 0.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        ), Vec3::new(0.0, 0.0, 1.0));

        assert_eq!(Vec3::new(5.0, 0.0, 0.0).normalize(), Vec3::new(1.0, 0.0, 0.0));

        assert!(Vec3::rand_in_unit_sphere().len() < 1.0);

        assert_eq!(Vec3::new(0.5, -0.5, 0.0).reflect(&Vec3::new(0.0, 1.0, 0.0)), Vec3::new(0.5, 0.5, 0.0));
    }
}

pub type Pos3 = Vec3;
pub type Color = Vec3;
