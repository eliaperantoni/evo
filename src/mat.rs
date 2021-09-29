use crate::hittable::Hit;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Mat {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Mat for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let mut scatter_dir = hit.normal + Vec3::rand_on_unit_sphere();

        if scatter_dir.is_near_zero() {
            scatter_dir = hit.normal;
        }

        Some((self.albedo, Ray::new(hit.point, scatter_dir)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz: fuzz.max(1.0) }
    }
}

impl Mat for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected = ray.dir.normalize().reflect(&hit.normal) + self.fuzz * Vec3::rand_in_unit_sphere();

        if Vec3::dot(&reflected, &hit.normal) > 0.0 {
            Some((self.albedo, Ray::new(reflected, hit.point)))
        } else {
            None
        }
    }
}
