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
}
