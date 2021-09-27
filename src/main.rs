fn main() {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for j in (0..HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..WIDTH {
            let r = i as f64 / (WIDTH as f64 - 1f64);
            let g = j as f64 / (HEIGHT as f64 - 1f64);
            let b = 0.25;

            let r = (255.999f64 * r) as u32;
            let g = (255.999f64 * g) as u32;
            let b = (255.999f64 * b) as u32;

            println!("{} {} {}", r, g, b);
        }
    }

    eprintln!("Done!");
}
