use std::io::Write;

use nalgebra::{Point3, Vector3};

pub type Vector = Vector3<f64>;
pub type Point = Point3<f64>;
pub type Color = Point3<f64>;

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
        cast_pixel(color.x),
        cast_pixel(color.y),
        cast_pixel(color.z),
    )?;
    Ok(())
}
