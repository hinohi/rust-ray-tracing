mod geo;

use std::io::Write;

pub use crate::geo::*;

pub type Color = Vector;
pub const RED: Color = Color::new(1.0, 0.0, 0.0);

pub fn cast_pixel(v: f64) -> u8 {
    if v < 0.0 {
        0
    } else if v > 1.0 {
        255
    } else {
        (v * 255.999).floor() as u8
    }
}

pub fn write_color<W: Write>(writer: &mut W, color: &Color) -> std::io::Result<()> {
    writeln!(
        writer,
        "{} {} {}",
        cast_pixel(color.x()),
        cast_pixel(color.y()),
        cast_pixel(color.z()),
    )?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn background(&self) -> Color {
        let t = (self.direction.y() / self.direction.norm() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Vector,
    radius: f64,
}

impl Sphere {
    pub const fn new(center: Vector, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    /// Calculate if the ray will hit the sphere
    ///
    /// Ray $\vec{R}(t) = \vec{O} + t\vec{D}$,
    /// center of sphere $\vec{C}$, radius of sphere $r$, then
    ///
    /// $$
    /// \begin{aligned}
    ///  r^2 &= \left(\vec{R}(t)-\vec{C}\right)^2 \\\\
    ///      &= \left(\vec{O} + t\vec{D} - \vec{C}\right)^2 \\\\
    /// t^2|D|^2 - 2t\vec{D}\cdot\overrightarrow{CO} + |\overrightarrow{CO}|^2 - r^2 &= 0
    /// \end{aligned}
    /// $$
    ///
    /// Since this is a simple quadratic equation,we can use
    /// the discriminant formula to determine the presence or absence of a root.
    pub fn is_hit(&self, ray: &Ray) -> bool {
        let co = self.center - ray.origin;
        let a = ray.direction.norm_squared();
        let b2 = ray.direction.dot(&co);
        let c = co.norm_squared() - self.radius * self.radius;
        let discriminant = b2 * b2 - a * c;
        discriminant > 0.0
    }
}
