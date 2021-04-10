mod geo;

use std::io::Write;

pub use crate::geo::*;

pub type Color = Vector;

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
