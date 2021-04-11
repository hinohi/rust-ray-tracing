use std::io::{stdout, Write};

use rand::Rng;
use ray_tracing::{write_color, Camera, Color, Hit, Ray, Sphere, Vector, BLACK};

fn ray_color<R: Rng>(rng: &mut R, ray: &Ray, objects: &[Sphere], depth: u32) -> Color {
    if depth == 0 {
        return BLACK;
    }
    for o in objects.iter() {
        if let Some(h) = o.hit(&ray) {
            let target = h.point + h.normal + Vector::random_in_unit_sphere(rng);
            let ray = Ray::new(h.point, target - h.point);
            return ray_color(rng, &ray, objects, depth - 1) * 0.5;
        }
    }
    ray.background()
}

fn main() {
    let stdout = stdout();
    let mut cout = stdout.lock();
    let mut rng = rand_pcg::Mcg128Xsl64::new(1);

    // camera
    let camera = Camera::default();

    // image
    let width = 400_u32;
    let height = (width as f64 / camera.aspect_ratio()).floor() as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // objects
    let world = vec![
        Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0),
    ];

    // render
    writeln!(cout, "P3").unwrap();
    writeln!(cout, "{} {}", width, height).unwrap();
    writeln!(cout, "255").unwrap();
    for y in (0..height).rev() {
        for x in 0..width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rng.gen::<f64>()) / (width as f64 - 1.0);
                let v = (y as f64 + rng.gen::<f64>()) / (height as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                color += ray_color(&mut rng, &ray, &world, max_depth);
            }
            write_color(&mut cout, color / samples_per_pixel as f64).unwrap()
        }
    }
}
