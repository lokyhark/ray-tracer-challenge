//! The Ray Tracer Challenge library.

// rustc lints
#![forbid(unsafe_code)]
#![deny(warnings)]
#![deny(missing_docs)]

pub(crate) mod util;

mod canvas;
pub use canvas::Canvas;

mod color;
pub use color::Color;

mod point;
pub use point::Point;

mod vector;
pub use vector::Vector;
