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
}
