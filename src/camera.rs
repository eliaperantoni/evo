use crate::ASPECT_RATIO;
use crate::ray::Ray;
use crate::vec3::{Pos3, Vec3};

pub struct Camera {
    eye: Pos3,
    lower_left_corner: Pos3,
    horizontal: Vec3,
    vertical: Vec3,
}

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f64 = 1.0;

impl Default for Camera {
    fn default() -> Self {
        let eye = Vec3::default();
        let horizontal = VIEWPORT_WIDTH * Vec3::x();
        let vertical = VIEWPORT_HEIGHT * Vec3::y();

        let lower_left_corner = eye - Vec3::z() * FOCAL_LENGTH - horizontal / 2.0 - vertical / 2.0;

        Self { eye, horizontal, vertical, lower_left_corner }
    }
}

impl Camera {
    pub fn make_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.eye, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.eye)
    }
}
