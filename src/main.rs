use vec3::*;

mod vec3;

fn main() {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for j in (0..HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..WIDTH {
            let col = Color::new(
                i as f64 / (WIDTH as f64 - 1f64),
                j as f64 / (HEIGHT as f64 - 1f64),
                0.25,
            );
            col.print();
        }
    }

    eprintln!("Done!");
}
