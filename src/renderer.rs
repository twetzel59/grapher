//! This module provides a system for rendering
//! graphable figures.

use sfml::graphics::{Color, PrimitiveType, Vertex, VertexArray};
use graphable::GraphableFn;

/// Stores a rendered figure that can be
/// drawn to the screen.
pub struct RenderCache {
    vtx_arr: VertexArray,
}

impl RenderCache {
    /// Perform a render of the figure and
    /// return the rendered figure encapsulated
    /// in an instance of ``RenderCache``.
    /// The function ``figure`` is evaluated for
    /// the interval ``[beg, end)``.
    pub fn render(figure: &GraphableFn, beg: u32, end: u32,
                  x_stretch: f32, y_stretch: f32, win_height: u32) -> RenderCache {
        let mut vtx_arr = VertexArray::new(PrimitiveType::LineStrip, 0);
        /*
        vtx_arr.append(&Vertex::with_pos_color((  0., 150.), Color::BLACK));
        vtx_arr.append(&Vertex::with_pos_color((100., 150.), Color::BLACK));
        vtx_arr.append(&Vertex::with_pos_color((150., 150.), Color::BLACK));
        vtx_arr.append(&Vertex::with_pos_color((150.,  30.), Color::BLACK));
        */

        let x_axis = win_height as f32 / 2.0;
        for i in beg..end {
            let x = i as f32;
            let point = (x, x_axis - figure.evaluate(x * x_stretch) * y_stretch);

            vtx_arr.append(&Vertex::with_pos_color(point, Color::BLACK));
        }

        RenderCache {
            vtx_arr,
        }
    }

    /// Return a reference to the cached
    /// inner ``VertexArray``.
    pub fn vtx_arr(&self) -> &VertexArray {
        &self.vtx_arr
    }
}
