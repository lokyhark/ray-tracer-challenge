use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{util::float_eq, Vector};

/// Geometric element of euclidian space identifiable by a tuple of coordinates
/// `(x,y,z)`.
#[derive(Copy, Clone, Debug, Default)]
pub struct Point {
    /// coordinate along the `x` axis
    pub x: f64,
    /// coordinate along the `y` axis
    pub y: f64,
    /// coordinate along the `z` axis
    pub z: f64,
}

impl Point {
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
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Display for Point {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!("({},{},{})", self.x, self.y, self.z))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z
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

    #[test]
    fn add_assign_vector() {
        let mut point = Point::new(1., 2., 3.);
        let vector = Vector::new(1., 2., 3.);
        let result = Point::new(2., 4., 6.);
        point += vector;
        assert_eq!(point, result);
    }

    #[test]
    fn sub() {
        let point1 = Point::new(2., 4., 6.);
        let point2 = Point::new(1., 2., 3.);
        let result = Vector::new(1., 2., 3.);
        assert_eq!(point1 - point2, result)
    }

    #[test]
    fn sub_assign_vector() {
        let mut point = Point::new(2., 4., 6.);
        let vector = Vector::new(1., 2., 3.);
        let result = Point::new(1., 2., 3.);
        point -= vector;
        assert_eq!(point, result);
    }

    #[test]
    fn sub_vector() {
        let point = Point::new(2., 4., 6.);
        let vector = Vector::new(1., 2., 3.);
        let result = Point::new(1., 2., 3.);
        assert_eq!(point - vector, result)
    }
}
