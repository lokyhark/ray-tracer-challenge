//! The Ray Tracer Challenge library.

// rustc lints
#![forbid(unsafe_code)]
#![deny(warnings)]
#![deny(missing_docs)]

pub(crate) mod util;

mod num;
pub use num::Num;

mod point;
pub use point::Point;
