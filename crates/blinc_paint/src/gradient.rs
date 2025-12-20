//! Gradient fills - re-exported from blinc_core for unified type system

pub use blinc_core::{Gradient, GradientSpace, GradientSpread, GradientStop};

use crate::{Color, Point};

/// Create a simple linear gradient between two colors
pub fn linear_simple(start: Point, end: Point, from: Color, to: Color) -> Gradient {
    Gradient::linear(start, end, from, to)
}

/// Create a simple radial gradient between two colors
pub fn radial_simple(center: Point, radius: f32, from: Color, to: Color) -> Gradient {
    Gradient::radial(center, radius, from, to)
}

/// Create a conic/angular gradient between two colors
pub fn conic_simple(center: Point, from: Color, to: Color) -> Gradient {
    Gradient::conic(center, from, to)
}
