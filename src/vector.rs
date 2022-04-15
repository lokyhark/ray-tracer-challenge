use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
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

    /// Returns the length/magniture of the vector.
    pub fn len(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalizes the vector.
    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    /// Returns normalized version of the vector.
    pub fn normalized(&self) -> Self {
        let len = self.len();
        Vector {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
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

impl<Float: Num> Mul<Float> for Vector<Float> {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<Float: Num> Div<Float> for Vector<Float> {
    type Output = Self;

    fn div(self, rhs: Float) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
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

    #[test]
    fn mul_scalar() {
        let vector = Vector::new(1., 2., 3.);
        let scalar = 2.;
        let result = Vector::new(2., 4., 6.);
        assert_eq!(vector * scalar, result);
    }

    #[test]
    fn div_scalar() {
        let vector = Vector::new(2., 4., 6.);
        let scalar = 2.;
        let result = Vector::new(1., 2., 3.);
        assert_eq!(vector / scalar, result);
    }

    #[test]
    fn len() {
        let vector = Vector::new(1., 0., 0.);
        assert!(float_eq(vector.len(), 1.));
        let vector = Vector::new(0., 1., 0.);
        assert!(float_eq(vector.len(), 1.));
        let vector = Vector::new(0., 0., 1.);
        assert!(float_eq(vector.len(), 1.));
        let vector = Vector::new(1., 2., 3.);
        assert!(float_eq(vector.len(), 14.0.sqrt()));
        let vector = Vector::new(-1., -2., -3.);
        assert!(float_eq(vector.len(), 14.0.sqrt()));
    }

    #[test]
    fn normalization() {
        let mut vector = Vector::new(2., 0., 0.);
        let result = Vector::new(1., 0., 0.);
        assert_eq!(vector.normalized(), result);
        vector.normalize();
        assert_eq!(vector, result);
        assert!(float_eq(vector.len(), 1.));

        let mut vector = Vector::new(0., 2., 0.);
        let result = Vector::new(0., 1., 0.);
        assert_eq!(vector.normalized(), result);
        vector.normalize();
        assert_eq!(vector, result);
        assert!(float_eq(vector.len(), 1.));

        let mut vector = Vector::new(0., 0., 2.);
        let result = Vector::new(0., 0., 1.);
        assert_eq!(vector.normalized(), result);
        vector.normalize();
        assert_eq!(vector, result);
        assert!(float_eq(vector.len(), 1.));

        let mut vector = Vector::new(1., 2., 3.);
        let result = Vector::new(1. / 14.0.sqrt(), 2. / 14.0.sqrt(), 3. / 14.0.sqrt());
        assert_eq!(vector.normalized(), result);
        vector.normalize();
        assert_eq!(vector, result);
        assert!(float_eq(vector.len(), 1.));
    }
}
