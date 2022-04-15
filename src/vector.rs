use std::fmt::Display;

use crate::Num;

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
