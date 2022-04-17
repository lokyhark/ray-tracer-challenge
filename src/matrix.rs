use std::ops::Mul;

use crate::util::float_eq;

/// Matrix 4x4.
#[derive(Clone, Debug)]
pub struct Matrix {
    elements: [f64; 16],
}

impl Matrix {
    /// Creates a new matrix from specified elements
    pub fn new(elements: [f64; 16]) -> Self {
        Self { elements }
    }

    /// Get element.
    pub fn get(&self, row: usize, col: usize) -> &f64 {
        assert!(row < 4);
        assert!(col < 4);
        &self.elements[row * 4 + col]
    }

    /// Get mutable element.
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        assert!(row < 4);
        assert!(col < 4);
        &mut self.elements[row * 4 + col]
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
}
