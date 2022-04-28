use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::util::float_eq;

/// Geometric object that has magnitude and direction denoted by a tuple of
/// scalar components `(x,y,z)`.
#[derive(Copy, Clone, Debug, Default)]
pub struct Vector {
    /// scalar component along the `x` axis
    pub x: f64,
    /// scalar component along the `y` axis
    pub y: f64,
    /// scalar component along the `z` axis
    pub z: f64,
}

impl Vector {
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
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Returns the length/magniture of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Vector;
    /// let vector = Vector::new(1., 2., 3.);
    /// assert_eq!(vector.len(), 14_f64.sqrt());
    /// ```
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalizes the vector.
    ///
    /// # Examples
    ///  
    /// ```
    /// # use ray_tracer_challenge::Vector;
    /// let mut vector = Vector::new(4., 0., 0.);
    /// let normal = Vector::new(1., 0., 0.);
    /// vector.normalize();
    /// assert_eq!(vector, normal);
    /// ```
    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    /// Returns normalized version of the vector.
    ///
    /// # Examples
    ///  
    /// ```
    /// # use ray_tracer_challenge::Vector;
    /// let vector = Vector::new(4., 0., 0.);
    /// let actual = vector.normalized();
    /// let expected = Vector::new(1., 0., 0.);
    /// assert_eq!(actual, expected);
    /// ```
    pub fn normalized(&self) -> Self {
        let len = self.len();
        Vector {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    /// Returns the dot product between `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Vector;
    /// let a = Vector::new(1., 2., 3.);
    /// let b = Vector::new(2., 3., 4.);
    /// assert_eq!(a.dot(b), 20.);
    /// ```
    pub fn dot(&self, rhs: Vector) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Returns the cross product between `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Vector;
    /// let a = Vector::new(1., 2., 3.);
    /// let b = Vector::new(2., 3., 4.);
    /// assert_eq!(a.cross(b), Vector::new(-1., 2., -1.));
    /// assert_eq!(b.cross(a), Vector::new(1., -2., 1.));
    /// ```
    pub fn cross(&self, rhs: Vector) -> Vector {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Display for Vector {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!("({},{},{})", self.x, self.y, self.z))
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl From<(f64, f64, f64)> for Vector {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::float_eq;

    use super::*;

    #[test]
    fn new() {
        let vector = Vector::new(4.3, -4.2, 3.1);
        assert!(float_eq(vector.x, 4.3));
        assert!(float_eq(vector.y, -4.2));
        assert!(float_eq(vector.z, 3.1));
    }

    #[test]
    fn from() {
        let vector: Vector = (1., 2., 3.).into();
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
    fn add_assign() {
        let mut vector = Vector::new(1., 2., 3.);
        let result = Vector::new(2., 4., 6.);
        vector += vector;
        assert_eq!(vector, result);
    }

    #[test]
    fn sub() {
        let vector1 = Vector::new(2., 4., 6.);
        let vector2 = Vector::new(1., 2., 3.);
        let result = Vector::new(1., 2., 3.);
        assert_eq!(vector1 - vector2, result);
    }

    #[test]
    fn sub_assign() {
        let mut vector1 = Vector::new(2., 4., 6.);
        let vector2 = Vector::new(1., 2., 3.);
        let result = Vector::new(1., 2., 3.);
        vector1 -= vector2;
        assert_eq!(vector1, result);
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
    fn mul_assign_scalar() {
        let mut vector = Vector::new(1., 2., 3.);
        let scalar = 2.;
        let result = Vector::new(2., 4., 6.);
        vector *= scalar;
        assert_eq!(vector, result);
    }

    #[test]
    fn div_scalar() {
        let vector = Vector::new(2., 4., 6.);
        let scalar = 2.;
        let result = Vector::new(1., 2., 3.);
        assert_eq!(vector / scalar, result);
    }

    #[test]
    fn div_assign_scalar() {
        let mut vector = Vector::new(2., 4., 6.);
        let scalar = 2.;
        let result = Vector::new(1., 2., 3.);
        vector /= scalar;
        assert_eq!(vector, result);
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
        assert!(float_eq(vector.len(), 14f64.sqrt()));
        let vector = Vector::new(-1., -2., -3.);
        assert!(float_eq(vector.len(), 14f64.sqrt()));
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
        let result = Vector::new(1. / 14f64.sqrt(), 2. / 14f64.sqrt(), 3. / 14f64.sqrt());
        assert_eq!(vector.normalized(), result);
        vector.normalize();
        assert_eq!(vector, result);
        assert!(float_eq(vector.len(), 1.));
    }

    #[test]
    fn dot() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(2., 3., 4.);
        assert!(float_eq(v1.dot(v2), 20.))
    }

    #[test]
    fn cross() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(2., 3., 4.);
        let result = Vector::new(-1., 2., -1.);
        assert_eq!(v1.cross(v2), result);
        assert_eq!(v2.cross(v1), -result);
        let x = Vector::new(1., 0., 0.);
        let y = Vector::new(0., 1., 0.);
        let z = Vector::new(0., 0., 1.);
        assert_eq!(x.cross(y), z);
        assert_eq!(y.cross(z), x);
        assert_eq!(z.cross(x), y);
    }
}
