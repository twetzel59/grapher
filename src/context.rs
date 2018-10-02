//! This module provides the graphing window.
//! It provides an abstraction over the underlying
//! graphics library. Currently, SFML is used as the
//! backend, but this module attempts to expose a stable
//! and backend-independent interface to the windowing and
//! drawing systems.

use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{ContextSettings, Event, Key, VideoMode};
use primitive::Vec2;

/// Abstracted type representing the graphing window
/// and any low-level state associated with it.
pub struct Context {
    inner: RenderWindow,
}

impl Context {
    /// Create a new ``Context``.
    /// This results in the immediate creation of a window.
    pub fn new(size: impl Into<Vec2<u32>>,
               title: &str,
               antialiasing_level: u32,
               vsync: bool) -> Context {
        let size = size.into();

        let context_settings = ContextSettings {
            antialiasing_level,
            ..Default::default()
        };

        let mut inner = RenderWindow::new(VideoMode::new(size.x, size.y, 32),
                                          title,
                                          Default::default(),
                                          &context_settings);

        inner.set_vertical_sync_enabled(vsync);

        Context {
            inner,
        }
    }

    pub fn tick(&mut self) -> bool {
        let mut running = true;

        while let Some(ev) = self.inner.poll_event() {
            match ev {
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => running = false,
                    _ => {},
                },
                Event::Closed => running = false,
                _ => {},
            }
        }

        self.inner.clear(&Color::WHITE);
        self.inner.display();

        running
    }
}
