//! This module provides the graphing window.
//! It provides an abstraction over the underlying
//! graphics library. Currently, SFML is used as the
//! backend, but this module attempts to expose a stable
//! and backend-independent interface to the windowing and
//! drawing systems.

use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system::Vector2u;
use sfml::window::{ContextSettings, Event, Key, VideoMode};
use graphable::GraphableFn;
use primitive::Vec2;
use renderer::RenderCache;

const STRETCH: (f32, f32) = (0.01, 100.);

/// Abstracted type representing the graphing window
/// and any low-level state associated with it.
pub struct Context {
    inner: RenderWindow,
    figures: Vec<(Box<dyn GraphableFn>, RenderCache)>,
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
        self.test();
        self.inner.display();

        running
    }

    /// Add a figure, represented by a ``GraphableFn``,
    /// to the graph. The figure will be retained for the
    /// lifetime of the ``Context``.
    /// Calls to this function can be very expensive,
    /// as the function will be evaluated for all
    /// visible x values.
    pub fn add_figure(&mut self, figure: Box<dyn GraphableFn>) {
        /*
        for i in 0..self.inner.size().x {
            let i = i as f32;
            println!("({}, {})", i, figure.evaluate(i));
        }
        */

        let Vector2u { x: end, y: height } = self.inner.size();
        let render_cache = RenderCache::render(&*figure, 0, end, STRETCH.0, STRETCH.1, height);
        self.figures.push((figure, render_cache))
    }

    pub fn test(&mut self) {
        self.inner.draw_vertex_array(self.figures[0].1.vtx_arr(), Default::default())
    }
}
