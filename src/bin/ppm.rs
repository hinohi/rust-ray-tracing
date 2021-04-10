use std::io::{stdout, Write};

use ray_tracing::{write_color, Ray, Vector};

fn main() {
    let stdout = stdout();
    let mut cout = stdout.lock();

    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400_u32;
    let height = (width as f64 / aspect_ratio).floor() as u32;

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector::new(0.0, 0.0, 0.0);
    let horizontal = Vector::new(viewport_width, 0.0, 0.0);
    let vertical = Vector::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector::new(0.0, 0.0, focal_length);

    // render
    writeln!(cout, "P3").unwrap();
    writeln!(cout, "{} {}", width, height).unwrap();
    writeln!(cout, "255").unwrap();
    for y in (0..height).rev() {
        for x in 0..width {
            let u = x as f64 / (width as f64 - 1.0);
            let v = y as f64 / (height as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            write_color(&mut cout, &r.background()).unwrap();
        }
    }
}
