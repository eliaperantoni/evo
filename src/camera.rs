use crate::ray::Ray;
use crate::vec3::{Pos3, Vec3};

pub struct CameraOpts {
    pub vfov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_dist: f64,

    pub eye: Pos3,
    pub target: Pos3,
    pub global_up: Vec3,
}

pub struct Camera {
    lens_radius: f64,
    eye: Pos3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    lower_left_corner: Pos3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(opts: CameraOpts) -> Self {
        let h = f64::atan(opts.vfov.to_radians() / 2.0);

        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * opts.aspect_ratio;

        let forward = (opts.target - opts.eye).normalize();
        let right = Vec3::cross(&forward, &opts.global_up).normalize();
        let up = Vec3::cross(&right, &forward).normalize();

        let horizontal = opts.focus_dist * viewport_width * right;
        let vertical = opts.focus_dist * viewport_height * up;

        let lower_left_corner = opts.eye + opts.focus_dist * forward - horizontal / 2.0 - vertical / 2.0;

        Self {
            lens_radius: opts.aperture / 2.0,
            eye: opts.eye,
            forward,
            right,
            up,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

impl Camera {
    pub fn make_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Vec3::rand_in_unit_disk();
        let offset = self.right * rd.x + self.up * rd.y;

        let dir = self.lower_left_corner + u * self.horizontal + v * self.vertical - (self.eye + offset);
        Ray::new(self.eye + offset, dir)
    }
}
