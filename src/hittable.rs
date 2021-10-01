use std::borrow::Borrow;
use std::ops::{Deref, DerefMut, Range};

use crate::mat::Mat;
use crate::ray::Ray;
use crate::vec3::{Pos3, Vec3};

pub struct Hit<'hit> {
    pub point: Pos3,
    // Normal always points towards the camera
    pub normal: Vec3,
    pub t: f64,
    pub is_front_face: bool,
    pub mat: &'hit dyn Mat,
}

impl<'hit> Hit<'hit> {
    pub fn new(point: Pos3, ray: &Ray, outward_normal: Vec3, t: f64, mat: &'hit dyn Mat) -> Self {
        let is_front_face = Vec3::dot(&ray.dir, &outward_normal) < 0.0;
        let normal = if is_front_face { outward_normal } else { -outward_normal };

        Self { point, normal, t, is_front_face, mat }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, t_range: &Range<f64>, ray: &Ray) -> Option<Hit>;
}

#[derive(Default)]
pub struct HittableVec(Vec<Box<dyn Hittable>>);

impl Deref for HittableVec {
    type Target<'hv> = Vec<&'hv dyn Hittable>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'hv> DerefMut for HittableVec {
    fn deref_mut(&'hv mut self) -> &'hv mut Self::Target {
        &mut self.0
    }
}

impl Hittable for HittableVec {
    fn hit(&self, t_range: &Range<f64>, ray: &Ray) -> Option<Hit> {
        let mut t_range = t_range.clone();
        let mut best_hit = None;

        for hittable in &self.0 {
            let hittable: &dyn Hittable = hittable.borrow();

            if let Some(hit) = hittable.hit(&t_range, ray) {
                t_range.end = hit.t;
                best_hit = Some(hit);
            }
        }

        best_hit
    }
}
