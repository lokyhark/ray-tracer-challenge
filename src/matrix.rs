use std::ops::Mul;

use crate::{util::float_eq, Point, Vector};

/// Matrix 4x4.
#[derive(Copy, Clone, Debug, Default)]
pub struct Matrix {
    elements: [f64; 16],
}

impl Matrix {
    const IDENTITY: Self = Self::new([1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.]);

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

    /// Returns the determinant of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Matrix;
    /// let a = Matrix::new([
    ///     -2., -8., 3., 5.,
    ///     -3., 1., 7., 3.,
    ///     1., 2., -9., 6.,
    ///     -6., 7., 7., -9.,
    /// ]);
    /// assert_eq!(a.determinant(), -4071.);
    /// ```
    pub fn determinant(&self) -> f64 {
        det4(self.elements)
    }

    /// Returns `true` if the matrix is invertible.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Matrix;
    /// let a = Matrix::new([
    ///     6., 4., 4., 4.,
    ///     5., 5., 7., 6.,
    ///     4., -9.,  3., -7.,
    ///     9.,  1.,  7., -6.
    /// ]);
    /// assert_eq!(a.determinant(), -2120.);
    /// assert!(a.is_invertible());
    /// ```
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.
    }

    /// Returns the inverse of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Matrix;
    /// let a = Matrix::new([
    ///     -5., 2., 6., -8.,
    ///     1., -5., 1., 8.,
    ///     7., 7., -6., -7.,
    ///     1., -3., 7., 4.
    /// ]);
    /// let i = Matrix::new([
    ///      0.21805,  0.45113,  0.24060, -0.04511,
    ///     -0.80827, -1.45677, -0.44361,  0.52068,
    ///     -0.07895, -0.22368, -0.05263,  0.19737,
    ///     -0.52256, -0.81391, -0.30075,  0.30639,
    /// ]);
    /// assert_eq!(a.inverse(), i);
    /// ```
    pub fn inverse(&self) -> Matrix {
        inv(self.elements)
    }
}

fn inv(matrix: [f64; 16]) -> Matrix {
    let mut inverse = [0.; 16];
    let determinant = det4(matrix);
    for row in 0..4 {
        for col in 0..4 {
            let cofactor = cofactor4(matrix, row, col);
            inverse[col * 4 + row] = cofactor / determinant;
        }
    }
    Matrix::new(inverse)
}

fn det2(matrix: [f64; 4]) -> f64 {
    matrix[0] * matrix[3] - matrix[2] * matrix[1]
}

fn det3(matrix: [f64; 9]) -> f64 {
    let mut det = 0.;
    for col in 0..3 {
        det += matrix[col] * cofactor3(matrix, 0, col);
    }
    det
}

fn det4(matrix: [f64; 16]) -> f64 {
    let mut det = 0.;
    for col in 0..4 {
        det += matrix[col] * cofactor4(matrix, 0, col);
    }
    det
}

fn submat3(matrix: [f64; 9], row: usize, col: usize) -> [f64; 4] {
    assert!(row < 3);
    assert!(col < 3);
    let mut elements = [0.; 4];
    let mut iter = elements.iter_mut();
    for r in (0..3).filter(|r| *r != row) {
        for c in (0..3).filter(|c| *c != col) {
            *iter.next().unwrap() = matrix[3 * r + c];
        }
    }
    elements
}

fn submat4(matrix: [f64; 16], row: usize, col: usize) -> [f64; 9] {
    assert!(row < 4);
    assert!(col < 4);
    let mut elements = [0.; 9];
    let mut iter = elements.iter_mut();
    for r in (0..4).filter(|r| *r != row) {
        for c in (0..4).filter(|c| *c != col) {
            *iter.next().unwrap() = matrix[4 * r + c];
        }
    }
    elements
}

fn minor3(matrix: [f64; 9], row: usize, col: usize) -> f64 {
    let sub = submat3(matrix, row, col);
    det2(sub)
}

fn minor4(matrix: [f64; 16], row: usize, col: usize) -> f64 {
    let sub = submat4(matrix, row, col);
    det3(sub)
}

fn cofactor3(matrix: [f64; 9], row: usize, col: usize) -> f64 {
    let minor = minor3(matrix, row, col);
    if (row + col) & 1 == 1 {
        -minor
    } else {
        minor
    }
}

fn cofactor4(matrix: [f64; 16], row: usize, col: usize) -> f64 {
    let minor = minor4(matrix, row, col);
    if (row + col) & 1 == 1 {
        -minor
    } else {
        minor
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
        let x = self.get(0, 0) * rhs.x + self.get(0, 1) * rhs.y + self.get(0, 2) * rhs.z + self.get(0, 3);
        let y = self.get(1, 0) * rhs.x + self.get(1, 1) * rhs.y + self.get(1, 2) * rhs.z + self.get(1, 3);
        let z = self.get(2, 0) * rhs.x + self.get(2, 1) * rhs.y + self.get(2, 2) * rhs.z + self.get(2, 3);
        Point::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::float_eq;

    use super::*;

    #[test]
    fn new() {
        #[rustfmt::skip]
        let matrix = Matrix::new([
            1., 2., 3., 4., 
            5.5, 6.5, 7.5, 8.5, 
            9., 10., 11., 12., 
            13.5, 14.5, 15.5, 16.5,
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
        #[rustfmt::skip]
        let a = Matrix::new([
            1., 2., 3., 4., 
            5., 6., 7., 8., 
            9., 8., 7., 6., 
            5., 4., 3., 2.
        ]);
        #[rustfmt::skip]
        let b = Matrix::new([
            1., 2., 3., 4., 
            5., 6., 7., 8., 
            9., 8., 7., 6., 
            5., 4., 3., 2.
        ]);
        #[rustfmt::skip]
        let c = Matrix::new([
            2., 3., 4., 5., 
            6., 7., 8., 9., 
            8., 7., 6., 5., 
            4., 3., 2., 1.
        ]);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn mul() {
        #[rustfmt::skip]
        let a = Matrix::new([
            1., 2., 3., 4., 
            5., 6., 7., 8., 
            9., 8., 7., 6., 
            5., 4., 3., 2.
        ]);
        #[rustfmt::skip]
        let b = Matrix::new([
            -2., 1., 2., 3., 
            3., 2., 1., -1., 
            4., 3., 6., 5., 
            1., 2., 7., 8.
        ]);
        #[rustfmt::skip]
        let c = Matrix::new([
            20., 22., 50., 48., 
            44., 54., 114., 108., 
            40., 58., 110., 102., 
            16., 26., 46., 42.,
        ]);
        assert_eq!(a * b, c);
    }

    #[test]
    fn mul_vector() {
        #[rustfmt::skip]
        let a = Matrix::new([
            1., 2., 3., 4., 
            2., 4., 4., 2., 
            8., 6., 4., 1., 
            0., 0., 0., 1.
        ]);
        let v = Vector::new(1., 2., 3.);
        let result = Vector::new(14., 22., 32.);
        assert_eq!(a * v, result);
    }

    #[test]
    fn mul_point() {
        #[rustfmt::skip]
        let a = Matrix::new([
            1., 2., 3., 4., 
            2., 4., 4., 2., 
            8., 6., 4., 1., 
            0., 0., 0., 1.
        ]);
        let p = Point::new(1., 2., 3.);
        let result = Point::new(18., 24., 33.);
        assert_eq!(a * p, result);
    }

    #[test]
    fn identity() {
        #[rustfmt::skip]
        let a = Matrix::new([
            0., 1., 2., 4., 
            1., 2., 4., 8., 
            2., 4., 8., 16., 
            4., 8., 16., 32.
        ]);
        assert_eq!(a * Matrix::identity(), a);
    }

    #[test]
    fn transpose() {
        #[rustfmt::skip]
        let a = Matrix::new([
            0., 9., 3., 0., 
            9., 8., 0., 8., 
            1., 8., 5., 3., 
            0., 0., 5., 8.
        ]);
        #[rustfmt::skip]
        let t = Matrix::new([
            0., 9., 1., 0., 
            9., 8., 8., 0., 
            3., 0., 5., 5., 
            0., 8., 3., 8.
        ]);
        assert_eq!(a.transpose(), t);
    }

    #[test]
    fn det2() {
        let matrix = [1., 5., -3., 2.];
        assert!(float_eq(super::det2(matrix), 17.));
    }

    #[test]
    fn submat3() {
        #[rustfmt::skip]
        let matrix = [
            1., 5., 0., 
            -3., 2., 7., 
            0., 6., -3.
        ];
        #[rustfmt::skip]
        let result = [
            -3., 2., 
            0., 6.
        ];
        assert_eq!(super::submat3(matrix, 0, 2), result);
    }

    #[test]
    fn submat4() {
        #[rustfmt::skip]
        let matrix = [
            -6., 1., 1., 6., 
            -8., 5., 8., 6., 
            -1., 0., 8., 2., 
            -7., 1., -1., 1.
        ];
        #[rustfmt::skip]
        let result = [
            -6., 1., 6., 
            -8., 8., 6., 
            -7., -1., 1.
        ];
        assert_eq!(super::submat4(matrix, 2, 1), result);
    }

    #[test]
    fn minor3() {
        #[rustfmt::skip]
        let matrix = [
            3., 5., 0., 
            2., -1., -7., 
            6., -1., 5.
        ];
        assert!(float_eq(super::minor3(matrix, 1, 0), 25.));
    }

    #[test]
    fn cofactor3() {
        #[rustfmt::skip]
        let matrix = [
            3., 5., 0., 
            2., -1., -7., 
            6., -1., 5.
        ];
        assert!(float_eq(super::cofactor3(matrix, 0, 0), -12.));
        assert!(float_eq(super::cofactor3(matrix, 1, 0), -25.));
    }

    #[test]
    fn cofactor4() {
        #[rustfmt::skip]
        let matrix = [
            -2., -8., 3., 5., 
            -3., 1., 7., 3., 
            1., 2., -9., 6., 
            -6., 7., 7., -9.
        ];
        assert!(float_eq(super::cofactor4(matrix, 0, 0), 690.));
        assert!(float_eq(super::cofactor4(matrix, 0, 1), 447.));
        assert!(float_eq(super::cofactor4(matrix, 0, 2), 210.));
        assert!(float_eq(super::cofactor4(matrix, 0, 3), 51.));
    }

    #[test]
    fn det3() {
        #[rustfmt::skip]
        let matrix = [
            1., 2., 6., 
            -5., 8., -4., 
            2., 6., 4.
        ];
        assert!(float_eq(super::det3(matrix), -196.));
    }

    #[test]
    fn det4() {
        #[rustfmt::skip]
        let elements = [
            -2., -8., 3., 5., 
            -3., 1., 7., 3., 
            1., 2., -9., 6., 
            -6., 7., 7., -9.
        ];
        let matrix = Matrix::new(elements);
        assert!(float_eq(matrix.determinant(), -4071.))
    }

    #[test]
    fn is_invertible() {
        #[rustfmt::skip]
        let matrix = Matrix::new([
            6., 4., 4., 4.,
            5., 5., 7., 6.,
            4., -9.,  3., -7.,
            9.,  1.,  7., -6.
        ]);
        assert!(matrix.is_invertible());
        #[rustfmt::skip]
        let matrix = Matrix::new([
            -4., 2., -2., -3.,
            9., 6., 2., 6.,
            0., -5., 1., -5.,
            0., 0., 0., 0.
        ]);
        assert!(!matrix.is_invertible());
    }

    #[test]
    fn invert() {
        #[rustfmt::skip]
        let matrix = Matrix::new([
            -5., 2., 6., -8.,
            1., -5., 1., 8., 
            7., 7., -6., -7.,
            1., -3., 7., 4.
        ]);
        #[rustfmt::skip]
        let inverse = Matrix::new([
             0.21805,  0.45113,  0.24060, -0.04511, 
            -0.80827, -1.45677, -0.44361,  0.52068, 
            -0.07895, -0.22368, -0.05263,  0.19737, 
            -0.52256, -0.81391, -0.30075,  0.30639,
        ]);
        assert_eq!(matrix.inverse(), inverse);

        #[rustfmt::skip]
        let matrix = Matrix::new([
            8., -5., 9., 2.,
            7., 5., 6., 1.,
            -6., 0., 9., 6.,
            -3., 0., -9., -4.
        ]);
        #[rustfmt::skip]
        let inverse = Matrix::new([
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
             0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308 
        ]);
        assert_eq!(matrix.inverse(), inverse);

        #[rustfmt::skip]
        let matrix = Matrix::new([
            9., 3., 0., 9.,
            -5., -2., -6., -3.,
            -4., 9., 6., 4.,
            -7., 6., 6., 2.,
        ]);
        #[rustfmt::skip]
        let inverse = Matrix::new([
            -0.04074, -0.07778, 0.14444, -0.22222, 
            -0.07778, 0.03333, 0.36667, -0.33333, 
            -0.02901, -0.14630, -0.10926, 0.12963, 
            0.17778, 0.06667, -0.26667, 0.33333, 
        ]);
        assert_eq!(matrix.inverse(), inverse);
    }

    #[test]
    fn inverse_mul() {
        #[rustfmt::skip]
        let a = Matrix::new([
             3.,-9., 7., 3., 
             3.,-8., 2.,-9., 
            -4., 4., 4., 1., 
            -6., 5.,-1., 1.,
        ]);
        let i = a.inverse();
        let identity = Matrix::identity();
        assert_eq!(a * i, identity);
    }

    #[test]
    fn inverse_product_mul() {
        #[rustfmt::skip]
        let a = Matrix::new([
             3.,-9., 7., 3., 
             3.,-8., 2.,-9., 
            -4., 4., 4., 1., 
            -6., 5.,-1., 1.,
        ]);
        #[rustfmt::skip]
        let b = Matrix::new([
            8.,2.,2.,2.,
            3.,-1., 7., 0., 
            7.,0.,5.,4.,
            6.,-2., 0., 5.,
        ]);
        let c = a * b;
        assert_eq!(c * b.inverse(), a);
    }

    #[test]
    fn invert_identity() {
        let identity = Matrix::identity();
        assert_eq!(identity.inverse(), identity);
    }

    #[test]
    fn transpose_inverse() {
        #[rustfmt::skip]
        let matrix = Matrix::new([
            -5., 2., 6., -8.,
            1., -5., 1., 8., 
            7., 7., -6., -7.,
            1., -3., 7., 4.
        ]);
        let inverse = matrix.inverse();
        let left = inverse.transpose();
        let transpose = matrix.transpose();
        let right = transpose.inverse();
        assert_eq!(left, right);
    }
}
