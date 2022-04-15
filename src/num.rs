use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

/// Marker trait for floating point numeric numbers.
pub trait Num:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Neg<Output = Self>
    + Sized
    + Clone
    + Copy
    + Debug
    + Display
    + PartialEq
    + PartialOrd
{
    const EPSILON: Self;

    fn abs(&self) -> Self;
}

impl Num for f32 {
    const EPSILON: Self = 0.00001_f32;

    fn abs(&self) -> Self {
        Self::abs(*self)
    }
}
impl Num for f64 {
    const EPSILON: Self = 0.00001_f64;

    fn abs(&self) -> Self {
        Self::abs(*self)
    }
}
