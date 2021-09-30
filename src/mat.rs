use crate::hittable::Hit;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use rand::Rng;

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
        Metal { albedo, fuzz: fuzz.min(1.0) }
    }
}

impl Mat for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected = ray.dir.normalize().reflect(&hit.normal) + self.fuzz * Vec3::rand_in_unit_sphere();

        if Vec3::dot(&reflected, &hit.normal) > 0.0 {
            Some((self.albedo, Ray::new(hit.point, reflected)))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    fn reflectance(cos_theta: f64, ir_ratio: f64) -> f64 {
        let r0 = ((1.0-ir_ratio) / (1.0+ir_ratio)).powi(2);
        r0 + (1.0-r0)*(1.0-cos_theta).powi(5)
    }
}

impl Mat for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let ir_ratio = if hit.is_front_face { 1.0 / self.ir } else { self.ir };

        let ray_dir = ray.dir.normalize();

        let cos_theta = Vec3::dot(&-ray_dir, &hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let scatter_dir;

        if ir_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, ir_ratio) > rand::thread_rng().gen::<f64>() {
            scatter_dir = ray_dir.reflect(&hit.normal);
        } else {
            scatter_dir = ray_dir.refract(&hit.normal, ir_ratio);
        }

        Some((Color::ones(), Ray::new(hit.point, scatter_dir)))
    }
}
