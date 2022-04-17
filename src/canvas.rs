use std::fmt::Write;

use crate::Color;

/// Grid of pixels.
#[derive(Clone, Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    /// Creates a canvas of specified size `(width, height)`.
    ///
    /// Each pixel have a black [`Color`].
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0., 0., 0.); width * height],
        }
    }

    /// Creates a canvas of specified size `(width, height)` and color.
    pub fn with_color(width: usize, height: usize, color: Color) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    /// Returns [`Color`] of specified pixel.
    pub fn get(&self, x: usize, y: usize) -> Option<&Color> {
        let index = x + y * self.width;
        self.pixels.get(index)
    }

    /// Returns mutable [`Color`] of specified pixel.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        let index = x + y * self.width;
        self.pixels.get_mut(index)
    }

    /// Get the canvas's width.
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the canvas's height.
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Iterator over pixels
    pub fn iter(&self) -> Iter {
        Iter(self.pixels.iter())
    }

    /// Mutable iterator over pixels
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut(self.pixels.iter_mut())
    }

    /// Turns canvas into PPM.
    pub fn ppm(&self) -> String {
        let mut ppm = String::new();
        writeln!(ppm, "P3").unwrap();
        writeln!(ppm, "{} {}", self.width, self.height).unwrap();
        writeln!(ppm, "255").unwrap();
        for h in 0..self.height {
            let mut colors = vec![];
            for w in 0..self.width {
                let color = self.get(w, h).unwrap();
                let (r, g, b) = color.as_tuple();
                colors.extend([r, g, b]);
            }
            let mut iter = colors.iter().peekable();
            let mut buf = String::new();
            while let Some(value) = iter.next() {
                let len = buf.len();
                write!(buf, "{}", value).unwrap();
                if buf.len() >= 70 {
                    writeln!(ppm, "{}", &buf[..len].trim()).unwrap();
                    buf.clear();
                    write!(buf, "{}", value).unwrap();
                }
                if iter.peek().is_some() {
                    buf.push(' ');
                }
            }
            if !buf.is_empty() {
                writeln!(ppm, "{}", buf).unwrap();
                buf.clear();
            }
        }
        ppm
    }
}

pub struct Iter<'a>(std::slice::Iter<'a, Color>);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Color;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct IterMut<'a>(std::slice::IterMut<'a, Color>);

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Color;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert!(canvas.pixels.iter().all(|p| *p == Color::new(0., 0., 0.)))
    }

    #[test]
    fn pixel() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1., 0., 0.);
        *canvas.get_mut(2, 3).unwrap() = red;
        assert_eq!(canvas.get(2, 3).unwrap(), &red);
    }

    #[test]
    fn ppm1() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0., 0.);
        let c2 = Color::new(0., 0.5, 0.);
        let c3 = Color::new(-0.5, 0., 1.);
        *canvas.get_mut(0, 0).unwrap() = c1;
        *canvas.get_mut(2, 1).unwrap() = c2;
        *canvas.get_mut(4, 2).unwrap() = c3;
        let expected = include_str!("../data/chapter_02_1.ppm");
        assert_eq!(canvas.ppm(), expected);
    }

    #[test]
    fn ppm2() {
        let mut canvas = Canvas::new(10, 2);
        let color = Color::new(1., 0.8, 0.6);
        for pixel in canvas.iter_mut() {
            *pixel = color;
        }
        let expected = include_str!("../data/chapter_02_2.ppm");
        assert_eq!(canvas.ppm(), expected);
    }
}
