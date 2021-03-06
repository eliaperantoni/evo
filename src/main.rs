#![feature(box_syntax)]
#![feature(type_ascription)]
#![feature(generic_associated_types)]

use std::borrow::Borrow;
use std::ops::Range;
use std::sync::{Arc, Mutex};
use std::thread;

use image::{Rgb, RgbImage};
use num::Integer;
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

const SAMPLES_PER_PIXEL: u32 = 50;

const MAX_DEPTH: u32 = 50;

const CHUNK_SIZE: u32 = 10;

fn main() {
    let eye = Pos3::new(13.0, 2.0, 3.0);
    let target = Pos3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(CameraOpts {
        vfov: 20.0,
        aspect_ratio: ASPECT_RATIO,
        aperture: 0.1,
        focus_dist: 10.0,
        eye,
        target,
        global_up: Pos3::y(),
    });

    let mut world = HittableVec::default();

    let mat_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let obj_ground = Sphere::new(Pos3::new(0.0, -1000.0, 0.0), 1000.0, box mat_ground);
    world.push(box obj_ground);

    let mut objs: Vec<Sphere> = Vec::new();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::thread_rng().gen::<f64>();
            let center = Pos3::new(
                a as f64 + 0.9 * rand::thread_rng().gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::thread_rng().gen::<f64>(),
            );

            if (center - Pos3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::rand() * Color::rand();
                    let mat = box Lambertian::new(albedo);
                    objs.push(Sphere::new(center, 0.2, mat));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::rand_range(0.5..=1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..=0.5);
                    let mat = box Metal::new(albedo, fuzz);
                    objs.push(Sphere::new(center, 0.2, mat));
                } else {
                    // Glass
                    let mat = box Dielectric::new(1.5);
                    objs.push(Sphere::new(center, 0.2, mat));
                }
            }
        }
    }

    for obj in objs {
        world.push(box obj);
    }

    let mat_1 = Dielectric::new(1.5);
    let obj_1 = Sphere::new(Pos3::new(0.0, 1.0, 0.0), 1.0, box mat_1);
    world.push(box obj_1);

    let mat_2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let obj_2 = Sphere::new(Pos3::new(-4.0, 1.0, 0.0), 1.0, box mat_2);
    world.push(box obj_2);

    let mat_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    let obj_3 = Sphere::new(Pos3::new(4.0, 1.0, 0.0), 1.0, box mat_3);
    world.push(box obj_3);

    struct Ctx {
        world: HittableVec,
        camera: Camera,
    }

    let ctx = Box::leak(box Ctx { world, camera });

    let n_chunks_y = IMG_HEIGHT.div_ceil(&CHUNK_SIZE);
    let n_chunks_x = IMG_WIDTH.div_ceil(&CHUNK_SIZE);

    let mut chunks: Vec<Vec<(u32, u32)>> = Vec::with_capacity((n_chunks_y * n_chunks_x) as usize);

    for chunk_y in 0..n_chunks_y {
        for chunk_x in 0..n_chunks_x {
            let mut chunk = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

            for y in chunk_y * CHUNK_SIZE..((chunk_y + 1) * CHUNK_SIZE).min(IMG_HEIGHT) {
                for x in chunk_x * CHUNK_SIZE..((chunk_x + 1) * CHUNK_SIZE).min(IMG_WIDTH) {
                    chunk.push((y, x));
                }
            }

            chunks.push(chunk);
        }
    }

    let chunks = Arc::new(Mutex::new(chunks));

    let buf = Arc::new(Mutex::new(RgbImage::new(IMG_WIDTH, IMG_HEIGHT)));

    let mut threads = Vec::new();

    for _ in 0..num_cpus::get() {
        let (chunks, buf) = (Arc::clone(&chunks.clone()), Arc::clone(&buf));
        let ctx = &*ctx;

        threads.push(thread::spawn(move || {
            let chunk = match chunks.lock().unwrap().pop() {
                Some(chunk) => chunk,
                None => return,
            };

            let mut rng = rand::thread_rng();

            for (y, x) in chunk {
                let mut pixel_color = Color::default();

                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (x as f64 + rng.gen::<f64>()) / (IMG_WIDTH - 1) as f64;
                    let v = (y as f64 + rng.gen::<f64>()) / (IMG_HEIGHT - 1) as f64;
                    let ray = ctx.camera.make_ray(u, v);
                    pixel_color += ray_color(&ray, &ctx.world, MAX_DEPTH);
                }

                pixel_color /= SAMPLES_PER_PIXEL as f64;

                let mut buf = buf.lock().unwrap();
                buf.put_pixel(x, IMG_HEIGHT - y - 1, Rgb(pixel_color.as_rgb_vec()));
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let buf = buf.lock().unwrap();
    buf.save("out.png").unwrap();

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

struct Sphere {
    center: Pos3,
    radius: f64,
    mat: Box<dyn Mat>,
}

impl Sphere {
    fn new(center: Pos3, radius: f64, mat: Box<dyn Mat>) -> Self {
        Sphere { center, radius, mat }
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
        let outward_normal = (hit_point - self.center) / self.radius;

        Some(Hit::new(hit_point, ray, outward_normal, t, self.mat.borrow()))
    }
}
