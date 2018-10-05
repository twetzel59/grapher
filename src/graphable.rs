//! This module provides the basics needed to
//! display a function on the graph.

use primitive::ColorRgb;

/// A trait for structs that implement the rendering of
/// a mathematical function.
pub trait GraphableFn {
    /// Evaluate the function for the given x, returning
    /// the y output.
    /// # Arguments:
    /// * ``x``: The x-coordinate to evaluate the ``GraphableFn`` for.
    /// # Return Value
    /// Return a tuple containing the y-coordinate (dependent variable)
    /// for the supplied ``x`` (independent variable) and the color of
    /// the point. If colors vary across evaluations, they will be
    /// interpolated.
    fn evaluate(&self, x: f32) -> (f32, ColorRgb);
}
