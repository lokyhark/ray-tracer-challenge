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
    + Default
    + Display
    + PartialEq
    + PartialOrd
{
    /// Absolute value error for equality.
    const EPSILON: Self;

    /// Returns the absolute value of `self`.
    fn abs(&self) -> Self;

    /// Returns the square root of `self`.
    fn sqrt(&self) -> Self;
}

impl Num for f32 {
    const EPSILON: Self = 1.0e-5_f32;

    fn abs(&self) -> Self {
        Self::abs(*self)
    }

    fn sqrt(&self) -> Self {
        Self::sqrt(*self)
    }
}

impl Num for f64 {
    const EPSILON: Self = 1.0e-5_f64;

    fn abs(&self) -> Self {
        Self::abs(*self)
    }

    fn sqrt(&self) -> Self {
        Self::sqrt(*self)
    }
}
