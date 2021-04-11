use rand::Rng;

use crate::{Color, HitPoint, Ray, Vector};

#[derive(Debug, Clone)]
pub enum Material {
    /// Lambertian reflectance
    Lambertian(Color),
    /// Metal
    Metal(Color),
}

impl Material {
    pub fn scatter<R: Rng>(&self, rng: &mut R, ray: &Ray, hit: &HitPoint) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian(color) => {
                let random = Vector::random_in_unit_sphere(rng);
                let mut direction = hit.normal + random / random.norm();
                if direction.norm_squared() < 1e-8 {
                    direction = hit.normal;
                }
                Some((Ray::new(hit.point, direction), *color))
            }
            Material::Metal(color) => {
                let direction = ray.direction / ray.direction.norm();
                let reflected = direction - hit.normal * (direction.dot(&hit.normal) * 2.0);
                if reflected.dot(&hit.normal) > 0.0 {
                    Some((Ray::new(hit.point, reflected), *color))
                } else {
                    None
                }
            }
        }
    }
}
