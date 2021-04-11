use crate::{Ray, Vector};

#[derive(Debug, Clone)]
pub struct Camera {
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    origin: Vector,
    horizontal: Vector,
    vertical: Vector,
    lower_left_corner: Vector,
}

impl Camera {
    pub fn new(
        origin: Vector,
        viewport_height: f64,
        viewport_width: f64,
        focal_length: f64,
    ) -> Camera {
        let horizontal = Vector::new(viewport_width, 0.0, 0.0);
        let vertical = Vector::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vector::new(0.0, 0.0, focal_length);
        Camera {
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.viewport_width / self.viewport_height
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        Camera::new(Vector::new(0.0, 0.0, 0.0), 2.0, 2.0 * aspect_ratio, 1.0)
    }
}
