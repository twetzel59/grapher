//! This module provides basic alternatives to
//! SFML's basic types (like vectors) to decouple
//! the grapher from the graphics library.

use sfml::system::Vector2;

/// A 2-vector.
#[derive(Clone, Copy, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    /// Create a new Vec2 from a tuple
    /// containing the ``(x, y)`` components.
    pub fn new(tuple: (T, T)) -> Self {
        Vec2 {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(tuple: (T, T)) -> Self {
        Self::new(tuple)
    }
}

impl<T> Into<Vector2<T>> for Vec2<T> {
    fn into(self) -> Vector2<T> {
        Vector2::new(self.x, self.y)
    }
}
