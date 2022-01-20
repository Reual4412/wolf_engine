//! Provides access to engine state and tooling.

mod game_loop_context;

pub use game_loop_context::*;
use winit::event_loop::EventLoop;

/// Provides a central hub through which to access all other contexts.
///
/// This is the main context.  It may be helpful to think of it as the "gateway" to the whole engine
/// because it owns all of the contexts active on the engine.  In many cases you will access
/// specific contexts through the main context, but sometimes the main context will have helper
/// functions for common tasks.
///
/// # Examples
///
/// Use the [ContextBuilder] to build a new `Context`.
///
/// ```
/// # use wolf_engine::ContextBuilder;
/// #
/// let (context, event_loop) = ContextBuilder::new()
///     // Insert additional settings here.    
///     .build();
/// ```
pub struct Context {
    pub game_loop: GameLoopContext,
}

/// Builds a [Context] object.
pub struct ContextBuilder {
    event_loop: Option<EventLoop<()>>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn without_event_loop() -> Self {
        Self {
            event_loop: None,
        }
    }

    /// Consumes the `ContextBuilder` and returns the built [Context] object.
    pub fn build(self) -> (Context, EventLoop<()>){
        let context = self.make_context();
        (context, self.event_loop.expect("No EventLoop was provided"))
    }

    pub fn build_without_event_loop(self) -> Context {
        self.make_context()
    }

    fn make_context(&self) -> Context {
        Context {
            game_loop: GameLoopContext::new(),
        }
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self {
            event_loop: Some(EventLoop::new())
        }
    }
}
