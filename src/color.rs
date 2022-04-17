use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::util::float_eq;

#[derive(Copy, Clone, Debug)]
/// Color identified by `(red, green, blue)` tuple.
pub struct Color {
    /// red
    pub r: f64,
    /// green
    pub g: f64,
    /// blue
    pub b: f64,
}

impl Color {
    /// Creates a new color from specified `(red, green, blue)`
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color {
            r: red,
            g: green,
            b: blue,
        }
    }
}

impl Display for Color {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!("({},{},{})", self.r, self.g, self.b))
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.r, other.r) && float_eq(self.g, other.g) && float_eq(self.b, other.b)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::util::float_eq;

    use super::*;

    #[test]
    fn new() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert!(float_eq(color.r, -0.5));
        assert!(float_eq(color.g, 0.4));
        assert!(float_eq(color.b, 1.7));
    }

    #[test]
    fn eq() {
        let color1 = Color::new(1., 2., 3.);
        let color2 = Color::new(1.000_001, 2.000_001, 3.000_001);
        assert_eq!(color1, color2);
    }

    #[test]
    fn add() {
        let color1 = Color::new(1., 2., 3.);
        let color2 = Color::new(2., 3., 4.);
        let result = Color::new(3., 5., 7.);
        assert_eq!(color1 + color2, result);
    }

    #[test]
    fn add_assign() {
        let mut color1 = Color::new(1., 2., 3.);
        let color2 = Color::new(2., 3., 4.);
        let result = Color::new(3., 5., 7.);
        color1 += color2;
        assert_eq!(color1, result);
    }

    #[test]
    fn sub() {
        let color1 = Color::new(2., 4., 6.);
        let color2 = Color::new(1., 2., 3.);
        let result = Color::new(1., 2., 3.);
        assert_eq!(color1 - color2, result);
    }

    #[test]
    fn sub_assign() {
        let mut color1 = Color::new(2., 4., 6.);
        let color2 = Color::new(1., 2., 3.);
        let result = Color::new(1., 2., 3.);
        color1 -= color2;
        assert_eq!(color1, result);
    }

    #[test]
    fn mul_scalar() {
        let color = Color::new(1., 2., 3.);
        let scalar = 2.;
        let result = Color::new(2., 4., 6.);
        assert_eq!(color * scalar, result);
    }

    #[test]
    fn mul_scalar_assign() {
        let mut color = Color::new(1., 2., 3.);
        let scalar = 2.;
        let result = Color::new(2., 4., 6.);
        color *= scalar;
        assert_eq!(color, result);
    }
}
