use crate::{Ray, Vector};

#[derive(Debug, Clone)]
pub struct Camera {
    origin: Vector,
    horizontal: Vector,
    vertical: Vector,
    lower_left_corner: Vector,
}

impl Camera {
    pub fn new(vertical_field_of_view: f64, aspect_ratio: f64) -> Camera {
        let h = (vertical_field_of_view.to_radians() * 0.5).tan();
        let viewport_height = h * 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let horizontal = Vector::new(viewport_width, 0.0, 0.0);
        let vertical = Vector::new(0.0, viewport_height, 0.0);
        let origin = Vector::new(0.0, 0.0, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vector::new(0.0, 0.0, 1.0);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.horizontal.norm() / self.vertical.norm()
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
        Camera::new(90.0, 16.0 / 9.0)
    }
}
