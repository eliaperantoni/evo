use std::ops;

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

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn print(&self) {
        let x = (255.999f64 * self.x) as u32;
        let y = (255.999f64 * self.y) as u32;
        let z = (255.999f64 * self.z) as u32;

        println!("{} {} {}", x, y, z);
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

    pub fn normalize(&self) -> Vec3 {
        *self / self.len()
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
    }
}

pub type Pos3 = Vec3;
pub type Color = Vec3;
