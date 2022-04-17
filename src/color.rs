use std::{
    fmt::Display,
    ops::{Add, AddAssign},
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
}
