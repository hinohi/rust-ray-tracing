use core::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector(f64, f64, f64);

macro_rules! impl_bin_op {
    ($trait_name:ident, $method_name:ident) => {
        impl $trait_name for Vector {
            type Output = Vector;
            fn $method_name(self, rhs: Vector) -> Vector {
                Vector(
                    self.0.$method_name(rhs.0),
                    self.1.$method_name(rhs.1),
                    self.2.$method_name(rhs.2),
                )
            }
        }
        impl $trait_name<f64> for Vector {
            type Output = Vector;
            fn $method_name(self, rhs: f64) -> Vector {
                Vector(
                    self.0.$method_name(rhs),
                    self.1.$method_name(rhs),
                    self.2.$method_name(rhs),
                )
            }
        }
    };
}

impl_bin_op!(Add, add);
impl_bin_op!(Sub, sub);
impl_bin_op!(Mul, mul);
impl_bin_op!(Div, div);

impl Vector {
    pub const fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector(x, y, z)
    }

    pub const fn x(&self) -> f64 {
        self.0
    }

    pub const fn y(&self) -> f64 {
        self.1
    }

    pub const fn z(&self) -> f64 {
        self.2
    }

    pub fn norm_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}
