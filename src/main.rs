use std::ops::Range;
use std::rc::Rc;

use vec3::*;
use ray::*;
use hittable::*;
use camera::*;
use rand::Rng;

mod vec3;
mod ray;
mod hittable;
mod camera;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: u32 = 400;
const IMG_HEIGHT: u32 = ((IMG_WIDTH as f64) / ASPECT_RATIO) as u32;

const SAMPLES_PER_PIXEL: u32 = 100;

fn main() {
    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    let camera = Camera::default();

    let mut world = HittableVec::default();
    world.push(Rc::new(Sphere::new(-1.0 * Vec3::z(), 0.5)));
    world.push(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut rng = rand::thread_rng();

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMG_WIDTH {
            let mut pixel_color = Color::default();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMG_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMG_HEIGHT - 1) as f64;
                let ray = camera.make_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            pixel_color /= SAMPLES_PER_PIXEL as f64;
            pixel_color.print();
        }
    }

    eprintln!("Done!");
}

fn ray_color(ray: &Ray, world: &HittableVec) -> Color {
    if let Some(hit) = world.hit(&(0.0..f64::INFINITY), ray) {
        return 0.5 * (hit.normal + Vec3::ones());
    }

    let dir = ray.dir.normalize();
    let t = (dir.y + 1.0) * 0.5;
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

struct Sphere {
    center: Pos3,
    radius: f64,
}

impl Sphere {
    fn new(center: Pos3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, t_range: &Range<f64>, ray: &Ray) -> Option<Hit> {
        let oc = ray.orig - self.center;

        let a = ray.dir.len_squared();
        let half_b = Vec3::dot(&ray.dir, &oc);
        let c = oc.len_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 { return None; }

        let roots = (
            (-half_b - discriminant.sqrt()) / a,
            (-half_b + discriminant.sqrt()) / a
        );

        let t = if t_range.contains(&roots.0) {
            roots.0
        } else if t_range.contains(&roots.1) {
            roots.1
        } else {
            return None;
        };

        let hit_point = ray.at(t);
        let outward_normal = (hit_point - self.center).normalize();

        Some(Hit::new(hit_point, ray, outward_normal, t))
    }
}
