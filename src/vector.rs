use std::{
    fmt::Display,
    ops::{Add, Neg, Sub},
};

use crate::{util::float_eq, Num};

/// Geometric object that has magnitude and direction denoted by a tuple of
/// scalar components `(x,y,z)`.
#[derive(Copy, Clone, Debug, Default)]
pub struct Vector<Float: Num> {
    /// scalar component along the `x` axis
    pub x: Float,
    /// scalar component along the `y` axis
    pub y: Float,
    /// scalar component along the `z` axis
    pub z: Float,
}

impl<Float: Num> Vector<Float> {
    /// Creates a `Vector` in euclidian solid space (three-dimensional) from
    /// specified scalar components.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Vector;
    /// let vector = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!(vector.x, 1.0);
    /// assert_eq!(vector.y, 2.0);
    /// assert_eq!(vector.z, 3.0);
    /// ```
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }
}

impl<Float: Num> Display for Vector<Float> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!("({},{},{})", self.x, self.y, self.z))
    }
}

impl<Float: Num> PartialEq for Vector<Float> {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
    }
}

impl<Float: Num> Add for Vector<Float> {
    type Output = Self;

    fn add(self, rhs: Vector<Float>) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<Float: Num> Sub for Vector<Float> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<Float: Num> Neg for Vector<Float> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::float_eq;

    use super::*;

    #[test]
    fn new() {
        let vector = Vector::new(1., 2., 3.);
        assert!(float_eq(vector.x, 1.));
        assert!(float_eq(vector.y, 2.));
        assert!(float_eq(vector.z, 3.));
    }

    #[test]
    fn display() {
        let vector = Vector::new(1.1, 2.2, 3.3);
        assert_eq!(vector.to_string(), "(1.1,2.2,3.3)");
    }

    #[test]
    fn partial_eq() {
        let p1 = Vector::new(1., 2., 3.);
        let p2 = Vector::new(1.000_001, 2.000_001, 3.000_001);
        assert_eq!(p1, p2)
    }

    #[test]
    fn add() {
        let vector = Vector::new(1., 2., 3.);
        let result = Vector::new(2., 4., 6.);
        assert_eq!(vector + vector, result);
    }

    #[test]
    fn sub() {
        let vector1 = Vector::new(2., 4., 6.);
        let vector2 = Vector::new(1., 2., 3.);
        let result = Vector::new(1., 2., 3.);
        assert_eq!(vector1 - vector2, result);
    }

    #[test]
    fn neg() {
        let vector = Vector::new(1., 2., 3.);
        let result = Vector::new(-1., -2., -3.);
        assert_eq!(-vector, result);
    }
}
