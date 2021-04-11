use std::io::{stdout, Write};

use rand::Rng;

use ray_tracing::*;

fn ray_color<R: Rng>(rng: &mut R, ray: &Ray, objects: &[Object], depth: u32) -> Color {
    if depth == 0 {
        return BLACK;
    }
    let mut hit: Option<(HitPoint, &Material)> = None;
    for o in objects.iter() {
        if let Some(new_hit) = o.sphere.hit(&ray) {
            if !matches!(hit, Some((ref now_hit, _)) if now_hit.t <= new_hit.t) {
                hit = Some((new_hit, &o.material));
            }
        }
    }
    if let Some((hit, mate)) = hit {
        if let Some((ray, attenuation)) = mate.scatter(rng, &ray, &hit) {
            attenuation * ray_color(rng, &ray, objects, depth - 1)
        } else {
            BLACK
        }
    } else {
        ray.background()
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    sphere: Sphere,
    material: Material,
}

fn main() {
    let stdout = stdout();
    let mut cout = stdout.lock();
    let mut rng = rand_pcg::Mcg128Xsl64::new(1);

    // camera
    let camera = CameraBuilder::new()
        .look_from(vec3!(3.0, 3.0, 2.0))
        .vertical_field_of_view(20.0)
        .blur(2.0);

    // image
    let width = 400_u32;
    let height = (width as f64 / camera.aspect_ratio()).floor() as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // objects
    let material_ground = Material::Lambertian {
        color: Color::new(0.8, 0.8, 0.0),
    };
    let material_a = Material::Lambertian {
        color: Color::new(0.1, 0.2, 0.5),
    };
    let material_b = Material::Dielectric {
        index_of_refraction: 1.5,
    };
    let material_c = Material::Metal {
        color: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.01,
    };

    let world = vec![
        Object {
            sphere: Sphere::new(vec3!(0.0, -100.5, -1.0), 100.0),
            material: material_ground,
        },
        Object {
            sphere: Sphere::new(vec3!(0.0, 0.0, -1.0), 0.5),
            material: material_a,
        },
        Object {
            sphere: Sphere::new(vec3!(-1.0, 0.0, -1.0), 0.5),
            material: material_b.clone(),
        },
        Object {
            sphere: Sphere::new(vec3!(-1.0, 0.0, -1.0), -0.45),
            material: material_b,
        },
        Object {
            sphere: Sphere::new(vec3!(1.0, 0.0, -1.0), 0.5),
            material: material_c,
        },
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
                let ray = camera.get_ray(&mut rng, u, v);
                color += ray_color(&mut rng, &ray, &world, max_depth);
            }
            write_color(&mut cout, color / samples_per_pixel as f64).unwrap()
        }
    }
}
