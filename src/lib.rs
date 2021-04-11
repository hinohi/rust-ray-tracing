mod camera;
mod geo;

use std::io::Write;

pub use crate::camera::*;
pub use crate::geo::*;

pub type Color = Vector;
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
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

pub fn write_color<W: Write>(writer: &mut W, color: Color) -> std::io::Result<()> {
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

    pub fn at(&self, t: f64) -> Vector {
        self.origin + self.direction * t
    }

    pub fn background(&self) -> Color {
        let t = (self.direction.y() / self.direction.norm() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray) -> Option<HitPoint>;
}

#[derive(Debug, Clone)]
pub struct HitPoint {
    pub point: Vector,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
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
}

impl Hit for Sphere {
    /// Calculate a hit point between given ray and this sphere
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
    /// Since this is a simple quadratic equation, just solve it.
    ///
    /// In general, a ray and a sphere intersect at two points.
    /// Since we take the z-direction to be from-to-front of the screen,
    /// the intersection point on the side visible to the camera is the
    /// side with the smaller t.
    ///
    /// Note: $\vec{D}\cdot\overrightarrow{CO} < 0$
    fn hit(&self, ray: &Ray) -> Option<HitPoint> {
        let co = self.center - ray.origin;
        let a = ray.direction.norm_squared();
        let b2 = ray.direction.dot(&co);
        let c = co.norm_squared() - self.radius * self.radius;
        let discriminant = b2 * b2 - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let t = (-b2 - discriminant.sqrt()) / a;
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
        if ray.direction.dot(&normal) <= 0.0 {
            Some(HitPoint {
                point,
                normal,
                t,
                front_face: true,
            })
        } else {
            Some(HitPoint {
                point,
                normal: -normal,
                t,
                front_face: false,
            })
        }
    }
}
