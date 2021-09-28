use std::ops::Range;
use std::rc::Rc;

use hittable::*;
use ray::*;
use vec3::*;

mod vec3;
mod ray;
mod hittable;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: u32 = 400;
    const IMG_HEIGHT: u32 = ((IMG_WIDTH as f64) / ASPECT_RATIO) as u32;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Vec3::default();
    let horizontal = Vec3::x() * VIEWPORT_WIDTH;
    let vertical = Vec3::y() * VIEWPORT_HEIGHT;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::z() * FOCAL_LENGTH;

    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    let mut world = HittableVec::default();
    world.push(Rc::new(Sphere::new(-1.0 * Vec3::z(), 0.5)));
    world.push(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMG_WIDTH {
            let u = i as f64 / (IMG_WIDTH - 1) as f64;
            let v = j as f64 / (IMG_HEIGHT - 1) as f64;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);

            let col = ray_color(&ray, &world);
            col.print();
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
