use std::{fmt::Display, ops::Add};

use crate::{util::float_eq, Num, Vector};

/// A geometric element of euclidian solid (three dimensional) space
/// identifiable by a tuple of coordinates `(x,y,z)`.
#[derive(Copy, Clone, Debug, Default)]
pub struct Point<Float: Num> {
    /// coordinate along the `x` axis
    pub x: Float,
    /// coordinate along the `y` axis
    pub y: Float,
    /// coordinate along the `z` axis
    pub z: Float,
}

impl<Float: Num> Point<Float> {
    /// Creates a `Point` in euclidian solid space (three-dimensional) from
    /// specified coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Point;
    /// let point = Point::new(1.0, 2.0, 3.0);
    /// assert_eq!(point.x, 1.0);
    /// assert_eq!(point.y, 2.0);
    /// assert_eq!(point.z, 3.0);
    /// ```
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }
}

impl<Float: Num> Display for Point<Float> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!("({},{},{})", self.x, self.y, self.z))
    }
}

impl<Float: Num> PartialEq for Point<Float> {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
    }
}

impl<Float: Num> Add<Vector<Float>> for Point<Float> {
    type Output = Self;

    fn add(self, rhs: Vector<Float>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::float_eq;

    use super::*;

    #[test]
    fn new() {
        let point = Point::new(1., 2., 3.);
        assert!(float_eq(point.x, 1.));
        assert!(float_eq(point.y, 2.));
        assert!(float_eq(point.z, 3.));
    }

    #[test]
    fn display() {
        let point = Point::new(1.1, 2.2, 3.3);
        assert_eq!(point.to_string(), "(1.1,2.2,3.3)");
    }

    #[test]
    fn partial_eq() {
        let p1 = Point::new(1., 2., 3.);
        let p2 = Point::new(1.000_001, 2.000_001, 3.000_001);
        assert_eq!(p1, p2)
    }

    #[test]
    fn add_vector() {
        let point = Point::new(1., 2., 3.);
        let vector = Vector::new(1., 2., 3.);
        let result = Point::new(2., 4., 6.);
        assert_eq!(point + vector, result);
    }
}
