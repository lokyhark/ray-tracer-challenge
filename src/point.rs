use std::fmt::Display;

use crate::{util::float_eq, Num};

/// Point in the euclidian space (3-dimension).
#[derive(Clone, Debug, Default)]
pub struct Point<Float: Num> {
    /// coordinate along the `x` axis
    pub x: Float,
    /// coordinate along the `y` axis
    pub y: Float,
    /// coordinate along the `z` axis
    pub z: Float,
}

impl<Float: Num> Point<Float> {
    /// Creates a `Point` in euclidian space (3-dimension) from specified
    /// coordinates.
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
        Point { x, y, z }
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
}
