//! This module provides the graphing window.
//! It provides an abstraction over the underlying
//! graphics library. Currently, SFML is used as the
//! backend, but this module attempts to expose a stable
//! and backend-independent interface to the windowing and
//! drawing systems.

use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{ContextSettings, Event, Key, VideoMode};
use graphable::GraphableFn;
use primitive::Vec2;

/// Abstracted type representing the graphing window
/// and any low-level state associated with it.
pub struct Context {
    inner: RenderWindow,
    figures: Vec<Box<dyn GraphableFn>>,
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
            figures: Vec::new(),
        }
    }

    /// Execute one frame of the main loop.
    /// Redrawing and event handling are performed.
    /// # Return value
    /// Returns ``true`` if the simulation should continue
    /// or ``false`` if the user wants to quit.
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

    /// Add a figure, represented by a ``GraphableFn``,
    /// to the graph. The figure will be retained indefinately.
    /// Calls to this function can be very expensive,
    /// as the function will be evaluated for all
    /// visible x values.
    pub fn add_figure(&mut self, figure: Box<dyn GraphableFn>) {
        for i in 0..self.inner.size().x {
            let i = i as f32;
            println!("({}, {})", i, figure.evaluate(i));
        }

        self.figures.push(figure)
    }
}
