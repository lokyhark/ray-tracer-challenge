use std::ops::Mul;

use crate::{util::float_eq, Point, Vector};

/// Matrix 4x4.
#[derive(Copy, Clone, Debug, Default)]
pub struct Matrix {
    elements: [f64; 16],
}

impl Matrix {
    const IDENTITY: Self = Self::new([
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ]);

    /// Creates a new matrix from specified elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Matrix;
    /// let matrix = Matrix::new([
    ///     1., 2., 3., 4.,
    ///     5.5, 6.5, 7.5, 8.5,
    ///     9., 10., 11., 12.,
    ///     13.5, 14.5, 15.5, 16.5,
    /// ]);
    /// assert_eq!(*matrix.get(0, 0), 1.);
    /// assert_eq!(*matrix.get(0, 1), 2.);
    /// assert_eq!(*matrix.get(0, 2), 3.);
    /// assert_eq!(*matrix.get(0, 3), 4.);
    /// assert_eq!(*matrix.get(1, 0), 5.5);
    /// assert_eq!(*matrix.get(1, 1), 6.5);
    /// assert_eq!(*matrix.get(1, 2), 7.5);
    /// assert_eq!(*matrix.get(1, 3), 8.5);
    /// assert_eq!(*matrix.get(2, 0), 9.);
    /// assert_eq!(*matrix.get(2, 1), 10.);
    /// assert_eq!(*matrix.get(2, 2), 11.);
    /// assert_eq!(*matrix.get(2, 3), 12.);
    /// assert_eq!(*matrix.get(3, 0), 13.5);
    /// assert_eq!(*matrix.get(3, 1), 14.5);
    /// assert_eq!(*matrix.get(3, 2), 15.5);
    /// assert_eq!(*matrix.get(3, 3), 16.5);
    /// ```
    pub const fn new(elements: [f64; 16]) -> Self {
        Self { elements }
    }

    /// Returns identity matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Matrix;
    /// let matrix = Matrix::identity();
    /// assert_eq!(*matrix.get(0, 0), 1.);
    /// assert_eq!(*matrix.get(0, 1), 0.);
    /// assert_eq!(*matrix.get(0, 2), 0.);
    /// assert_eq!(*matrix.get(0, 3), 0.);
    /// assert_eq!(*matrix.get(1, 0), 0.);
    /// assert_eq!(*matrix.get(1, 1), 1.);
    /// assert_eq!(*matrix.get(1, 2), 0.);
    /// assert_eq!(*matrix.get(1, 3), 0.);
    /// assert_eq!(*matrix.get(2, 0), 0.);
    /// assert_eq!(*matrix.get(2, 1), 0.);
    /// assert_eq!(*matrix.get(2, 2), 1.);
    /// assert_eq!(*matrix.get(2, 3), 0.);
    /// assert_eq!(*matrix.get(3, 0), 0.);
    /// assert_eq!(*matrix.get(3, 1), 0.);
    /// assert_eq!(*matrix.get(3, 2), 0.);
    /// assert_eq!(*matrix.get(3, 3), 1.);
    /// ```
    pub const fn identity() -> Self {
        Self::IDENTITY
    }

    /// Get element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Matrix;
    /// let matrix = Matrix::identity();
    /// assert_eq!(*matrix.get(0, 0), 1.);
    /// ```
    pub fn get(&self, row: usize, col: usize) -> &f64 {
        assert!(row < 4);
        assert!(col < 4);
        &self.elements[row * 4 + col]
    }

    /// Get mutable element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Matrix;
    /// let mut matrix = Matrix::identity();
    /// assert_eq!(*matrix.get(0, 0), 1.);
    /// *matrix.get_mut(0, 0) *= 2.;
    /// assert_eq!(*matrix.get(0, 0), 2.);
    /// ```
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        assert!(row < 4);
        assert!(col < 4);
        &mut self.elements[row * 4 + col]
    }

    /// Transpose the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Matrix;
    /// let a = Matrix::new([
    ///     0., 9., 3., 0.,
    ///     9., 8., 0., 8.,
    ///     1., 8., 5., 3.,
    ///     0., 0., 5., 8.,
    /// ]);
    /// let t = Matrix::new([
    ///     0., 9., 1., 0.,
    ///     9., 8., 8., 0.,
    ///     3., 0., 5., 5.,
    ///     0., 8., 3., 8.,
    /// ]);
    /// assert_eq!(a.transpose(), t);
    /// ```
    pub fn transpose(&self) -> Self {
        let mut matrix = Self::default();
        for row in 0..4 {
            for col in 0..4 {
                *matrix.get_mut(col, row) = *self.get(row, col);
            }
        }
        matrix
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.elements
            .iter()
            .zip(other.elements.iter())
            .all(|(&a, &b)| float_eq(a, b))
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut elements = [0.; 16];
        for row in 0..4 {
            for col in 0..4 {
                elements[row * 4 + col] = self.get(row, 0) * rhs.get(0, col)
                    + self.get(row, 1) * rhs.get(1, col)
                    + self.get(row, 2) * rhs.get(2, col)
                    + self.get(row, 3) * rhs.get(3, col);
            }
        }
        Matrix { elements }
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let x = self.get(0, 0) * rhs.x + self.get(0, 1) * rhs.y + self.get(0, 2) * rhs.z;
        let y = self.get(1, 0) * rhs.x + self.get(1, 1) * rhs.y + self.get(1, 2) * rhs.z;
        let z = self.get(2, 0) * rhs.x + self.get(2, 1) * rhs.y + self.get(2, 2) * rhs.z;
        Vector::new(x, y, z)
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let x = self.get(0, 0) * rhs.x
            + self.get(0, 1) * rhs.y
            + self.get(0, 2) * rhs.z
            + self.get(0, 3);
        let y = self.get(1, 0) * rhs.x
            + self.get(1, 1) * rhs.y
            + self.get(1, 2) * rhs.z
            + self.get(1, 3);
        let z = self.get(2, 0) * rhs.x
            + self.get(2, 1) * rhs.y
            + self.get(2, 2) * rhs.z
            + self.get(2, 3);
        Point::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::float_eq;

    use super::*;

    #[test]
    fn new() {
        let matrix = Matrix::new([
            1., 2., 3., 4., 5.5, 6.5, 7.5, 8.5, 9., 10., 11., 12., 13.5, 14.5, 15.5, 16.5,
        ]);
        assert!(float_eq(*matrix.get(0, 0), 1.));
        assert!(float_eq(*matrix.get(0, 1), 2.));
        assert!(float_eq(*matrix.get(0, 2), 3.));
        assert!(float_eq(*matrix.get(0, 3), 4.));
        assert!(float_eq(*matrix.get(1, 0), 5.5));
        assert!(float_eq(*matrix.get(1, 1), 6.5));
        assert!(float_eq(*matrix.get(1, 2), 7.5));
        assert!(float_eq(*matrix.get(1, 3), 8.5));
        assert!(float_eq(*matrix.get(2, 0), 9.));
        assert!(float_eq(*matrix.get(2, 1), 10.));
        assert!(float_eq(*matrix.get(2, 2), 11.));
        assert!(float_eq(*matrix.get(2, 3), 12.));
        assert!(float_eq(*matrix.get(3, 0), 13.5));
        assert!(float_eq(*matrix.get(3, 1), 14.5));
        assert!(float_eq(*matrix.get(3, 2), 15.5));
        assert!(float_eq(*matrix.get(3, 3), 16.5));
    }

    #[test]
    fn eq() {
        let a = Matrix::new([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);
        let b = Matrix::new([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);
        let c = Matrix::new([
            2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2., 1.,
        ]);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn mul() {
        let a = Matrix::new([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);
        let b = Matrix::new([
            -2., 1., 2., 3., 3., 2., 1., -1., 4., 3., 6., 5., 1., 2., 7., 8.,
        ]);
        let c = Matrix::new([
            20., 22., 50., 48., 44., 54., 114., 108., 40., 58., 110., 102., 16., 26., 46., 42.,
        ]);

        assert_eq!(a * b, c);
    }

    #[test]
    fn mul_vector() {
        let a = Matrix::new([
            1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.,
        ]);
        let v = Vector::new(1., 2., 3.);
        let result = Vector::new(14., 22., 32.);
        assert_eq!(a * v, result);
    }

    #[test]
    fn mul_point() {
        let a = Matrix::new([
            1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.,
        ]);
        let p = Point::new(1., 2., 3.);
        let result = Point::new(18., 24., 33.);
        assert_eq!(a * p, result);
    }

    #[test]
    fn identity() {
        let a = Matrix::new([
            0., 1., 2., 4., 1., 2., 4., 8., 2., 4., 8., 16., 4., 8., 16., 32.,
        ]);
        assert_eq!(a * Matrix::identity(), a);
    }

    #[test]
    fn transpose() {
        let a = Matrix::new([
            0., 9., 3., 0., 9., 8., 0., 8., 1., 8., 5., 3., 0., 0., 5., 8.,
        ]);
        let t = Matrix::new([
            0., 9., 1., 0., 9., 8., 8., 0., 3., 0., 5., 5., 0., 8., 3., 8.,
        ]);
        assert_eq!(a.transpose(), t);
    }
}
