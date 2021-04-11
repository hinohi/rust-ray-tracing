use rand::Rng;

use crate::{Color, HitPoint, Ray, Vector};

#[derive(Debug, Clone)]
pub enum Material {
    /// Lambertian reflectance
    Lambertian { color: Color },
    /// Metal, ray can reflect
    Metal { color: Color, fuzz: f64 },
    /// Dielectric, ray can through and flexion
    Dielectric { index_of_refraction: f64 },
}

impl Material {
    pub fn scatter<R: Rng>(&self, rng: &mut R, ray: &Ray, hit: &HitPoint) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian { color } => {
                let mut direction = hit.normal + Vector::random_in_unit_sphere(rng).normalized();
                if direction.norm_squared() < 1e-8 {
                    direction = hit.normal;
                }
                Some((Ray::new(hit.point, direction), *color))
            }
            Material::Metal { color, fuzz } => {
                let random = Vector::random_in_unit_sphere(rng);
                let reflected = reflect(ray.direction.normalized(), hit.normal) + random * *fuzz;
                if reflected.dot(&hit.normal) > 0.0 {
                    Some((Ray::new(hit.point, reflected), *color))
                } else {
                    None
                }
            }
            Material::Dielectric {
                index_of_refraction,
            } => {
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if hit.front_face {
                    1.0 / *index_of_refraction
                } else {
                    *index_of_refraction
                };
                let direction = ray.direction.normalized();
                let cos = (-direction.dot(&hit.normal)).min(1.0);
                let sin = (1.0 - cos * cos).sqrt();
                let direction = if refraction_ratio * sin > 1.0
                    || reflectance(cos, refraction_ratio) > rng.gen()
                {
                    reflect(direction, hit.normal)
                } else {
                    refract(direction, hit.normal, refraction_ratio)
                };
                Some((Ray::new(hit.point, direction), attenuation))
            }
        }
    }
}

fn reflect(v: Vector, n: Vector) -> Vector {
    v - n * (v.dot(&n) * 2.0)
}

fn refract(uv: Vector, n: Vector, refraction_ratio: f64) -> Vector {
    let cos = (-uv.dot(&n)).min(1.0);
    let r_out_prep = (uv + n * cos) * refraction_ratio;
    let r_out_parallel = n * (1.0 - r_out_prep.norm_squared()).abs().sqrt();
    r_out_prep - r_out_parallel
}

/// Schlick Approximation
fn reflectance(cos: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
