use ray::*;
use vec3::*;

mod vec3;
mod ray;

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
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::z()*FOCAL_LENGTH;

    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMG_WIDTH {
            let u = i as f64 / (IMG_WIDTH - 1) as f64;
            let v = j as f64 / (IMG_HEIGHT - 1) as f64;

            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);

            let col = ray_color(&ray);
            col.print();
        }
    }

    eprintln!("Done!");
}

fn ray_color(ray: &Ray) -> Color {
    let dir = ray.dir.normalize();
    let t = (dir.y + 1.0) * 0.5;
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
