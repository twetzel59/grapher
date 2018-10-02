//! This module provides the basics needed to
//! display a function on the graph.

/// A trait for structs that implement the rendering of
/// a mathematical function.
pub trait GraphableFn {
    /// Evaluate the function for the given x, returning
    /// the y output.
    /// # Arguments:
    /// * ``x``: The x-coordinate to evaluate the ``GraphableFn`` for.
    /// # Return Value
    /// Return the y-coordinate (dependent variable)
    /// for the supplied ``x`` (independent variable).
    fn evaluate(&self, x: f32) -> f32;
}
