use crate::{Hit, HitPoint, Ray, Vector};

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
        let co = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let b2 = ray.direction.dot(&co);
        let c = co.norm_squared() - self.radius * self.radius;
        let discriminant = b2.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let mut t = (-b2 - discriminant.sqrt()) / a;
        if t < 1e-6 {
            t = (-b2 + discriminant.sqrt()) / a;
            if t < 1e-6 {
                return None;
            }
        }
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
