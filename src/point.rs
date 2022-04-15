use crate::Num;

/// Point in the euclidian space (3-dimension).
#[derive(Clone, Debug)]
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
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Point { x, y, z }
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
}
