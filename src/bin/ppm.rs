use std::io::{stdout, Write};

use ray_tracing::{write_color, Color};

fn main() {
    let width = 256;
    let height = 256;

    let stdout = stdout();
    let mut cout = stdout.lock();
    writeln!(cout, "P3").unwrap();
    writeln!(cout, "{} {}", width, height).unwrap();
    writeln!(cout, "255").unwrap();
    for y in (0..height).rev() {
        for x in 0..width {
            let rgb = Color::new(
                x as f64 / (width as f64 - 1.0),
                y as f64 / (height as f64 - 1.0),
                0.25,
            );
            write_color(&mut cout, &rgb).unwrap();
        }
    }
}
