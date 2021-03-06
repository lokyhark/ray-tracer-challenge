use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::util::float_eq;

#[derive(Copy, Clone, Debug, Default)]
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Color;
    /// let color = Color::new(-0.5, 0.4, 1.7);
    /// assert_eq!(color.r, -0.5);
    /// assert_eq!(color.g, 0.4);
    /// assert_eq!(color.b, 1.7);
    /// ```
    pub const fn new(red: f64, green: f64, blue: f64) -> Self {
        Color {
            r: red,
            g: green,
            b: blue,
        }
    }

    /// Creates black color `(0., 0., 0.)`.
    pub const fn black() -> Self {
        const COLOR: Color = Color { r: 0., g: 0., b: 0. };
        COLOR
    }

    /// Creates white color `(1., 1., 1.)`.
    pub const fn white() -> Self {
        const COLOR: Color = Color { r: 1., g: 1., b: 1. };
        COLOR
    }

    /// Creates red color `(1., 0., 0.)`.
    pub const fn red() -> Self {
        const COLOR: Color = Color { r: 1., g: 0., b: 0. };
        COLOR
    }

    /// Creates green color `(0., 1., 0.)`.
    pub const fn green() -> Self {
        const COLOR: Color = Color { r: 0., g: 1., b: 0. };
        COLOR
    }

    /// Creates blue color `(0., 0., 1.)`.
    pub const fn blue() -> Self {
        const COLOR: Color = Color { r: 0., g: 0., b: 1. };
        COLOR
    }

    /// Converts color to u8 tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ray_tracer_challenge::Color;
    /// let color = Color::new(1., 0.5, 0.);
    /// assert_eq!(color.as_tuple(), (255, 128, 0));
    /// ```
    pub fn as_tuple(&self) -> (u8, u8, u8) {
        let r = (self.r.clamp(0., 1.) * 255.).round() as u8;
        let g = (self.g.clamp(0., 1.) * 255.).round() as u8;
        let b = (self.b.clamp(0., 1.) * 255.).round() as u8;
        (r, g, b)
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

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Self {
            r: tuple.0,
            g: tuple.1,
            b: tuple.2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::float_eq;

    use super::*;

    #[test]
    fn new() {
        let color = Color::new(1., 2., 3.);
        assert!(float_eq(color.r, 1.));
        assert!(float_eq(color.g, 2.));
        assert!(float_eq(color.b, 3.));
    }

    #[test]
    fn from() {
        let color: Color = (1., 2., 3.).into();
        assert!(float_eq(color.r, 1.));
        assert!(float_eq(color.g, 2.));
        assert!(float_eq(color.b, 3.));
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

    #[test]
    fn mul() {
        let color1 = Color::new(1., 0.2, 0.4);
        let color2 = Color::new(0.9, 1., 0.1);
        let result = Color::new(0.9, 0.2, 0.04);
        assert_eq!(color1 * color2, result);
    }

    #[test]
    fn mul_assign() {
        let mut color1 = Color::new(1., 0.2, 0.4);
        let color2 = Color::new(0.9, 1., 0.1);
        let result = Color::new(0.9, 0.2, 0.04);
        color1 *= color2;
        assert_eq!(color1, result);
    }
}
