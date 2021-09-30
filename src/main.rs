use std::ops::Range;

use rand::Rng;

use camera::*;
use hittable::*;
use mat::*;
use ray::*;
use vec3::*;

mod vec3;
mod ray;
mod hittable;
mod camera;
mod mat;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMG_WIDTH: u32 = 400;
const IMG_HEIGHT: u32 = ((IMG_WIDTH as f64) / ASPECT_RATIO) as u32;

const SAMPLES_PER_PIXEL: u32 = 100;

const MAX_DEPTH: u32 = 50;

fn main() {
    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    let camera = Camera::new(CameraOpts {
        vfov: 20.0,
        aspect_ratio: ASPECT_RATIO,
        eye: Pos3::new(-2.0, 2.0, 1.0),
        target: Pos3::new(0.0, 0.0, -1.0),
        global_up: Pos3::y(),
    });

    let mat_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let mat_left = Dielectric::new(1.5);
    let mat_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    let mut world = HittableVec::default();

    let objects = vec![
        Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, &mat_ground),
        Sphere::new(Vec3::new( 0.0,    0.0, -1.0),   0.5, &mat_center),
        Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.5, &mat_left),
        Sphere::new(Vec3::new(-1.0,    0.0, -1.0), -0.45, &mat_left),
        Sphere::new(Vec3::new( 1.0,    0.0, -1.0),   0.5, &mat_right),
    ];

    for object in &objects {
        world.push(object);
    }

    let mut rng = rand::thread_rng();

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMG_WIDTH {
            let mut pixel_color = Color::default();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMG_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMG_HEIGHT - 1) as f64;
                let ray = camera.make_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            pixel_color /= SAMPLES_PER_PIXEL as f64;
            pixel_color.print();
        }
    }

    eprintln!("Done!");
}

fn ray_color(ray: &Ray, world: &HittableVec, depth: u32) -> Color {
    if depth == 0 {
        return Color::default();
    }

    if let Some(hit) = world.hit(&(0.001..f64::INFINITY), ray) {
        return if let Some((color, scattered)) = hit.mat.scatter(ray, &hit) {
            color * ray_color(&scattered, world, depth - 1)
        } else {
            Color::default()
        };
    }

    let dir = ray.dir.normalize();
    let t = (dir.y + 1.0) * 0.5;
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

struct Sphere<'sphere> {
    center: Pos3,
    radius: f64,
    mat: &'sphere dyn Mat,
}

impl<'sphere> Sphere<'sphere> {
    fn new(center: Pos3, radius: f64, mat: &'sphere dyn Mat) -> Self {
        Sphere { center, radius, mat }
    }
}

impl<'sphere> Hittable for Sphere<'sphere> {
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
        let outward_normal = (hit_point - self.center) / self.radius;

        Some(Hit::new(hit_point, ray, outward_normal, t, self.mat))
    }
}
