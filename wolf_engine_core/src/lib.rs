//! Provides common tools, types, and functions for the engine.
//!
//! The Core API provides all the parts likely to be (re)used by other parts of the engine.  It
//! is mostly intended for those building, or making extensions to Wolf Engine, but there are some
//! tools for end-users as well.
//!
//! # Getting Started
//!
//! While it's possible to build games using the `core` module alone, this isn't recommended
//! unless you *really* know what you're doing, or you want to build your own, game-specific,
//! engine.
//!
//! The core module really doesn't do a lot on it's own.  It's closer to a collection of basic
//! tools than it is an actual game framework.  As such, you're expected to write your own
//! main-loop, and respond to events entirely on your own.
//!
//! Here's an example of a basic main-loop:
//!
//! ```
//! # use wolf_engine_core as wolf_engine;
//! use wolf_engine::prelude::*;
//!
//! pub struct GameData {
//!     pub number: i32,
//! }
//!
//! pub fn main() {
//!     // Start by initializing the engine's Event-Loop, and Context.
//!     let (mut event_loop, mut context) = wolf_engine::init(GameData { number: 0 });
//!     
//!     // The Event-Loop will continue to return events, every call, until a Quit event is sent,
//!     // only then, will the Event-Loop will return None.
//!     while let Some(event) = event_loop.next_event() {
//!         process_event(event, &mut context);
//!     }
//! }
//!
//! pub fn process_event(event: Event, context: &mut Context<GameData>) {
//!     match event {
//!         // Indicates there are no more events on the queue, or, essentially, the end of the
//!         // current frame.  You should put most of your game logic here.
//!         Event::EventsCleared => {
//!             if context.data.number == 3 {
//!                 context.quit();
//!             } else {
//!                 context.data.number += 1;
//!             }
//!             println!("{}", context.data.number);
//!         }
//!         // Shut down the game.
//!         Event::Quit => println!("Quit event received.  Goodbye!"),
//!         _ => (),
//!     }
//! }
//! ```
//!
//! You can use this example as a jumping-off point for your game.  Most of Wolf Engine's libraries
//! are built against `core`, so you can very likely pull in other modules and start using them
//! without to much trouble.
//!
//! You can also look in the
//! [examples folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples) for additional
//! examples.

mod context;
pub use context::*;
mod event_loop;
pub use event_loop::*;

pub mod events;

#[cfg(feature = "logging")]
pub mod logging;

#[doc(hidden)]
pub mod prelude {
    pub use super::*;
    pub use events::*;
}

/// Represents the [`EventLoop`]-[`Context`] pair that makes up "the engine."
pub type Engine<D> = (EventLoop, Context<D>);

/// Initializes a new instance of the [`EventLoop`], and its associated [`Context`], with the
/// provided data.
///
/// #  Examples
///
/// ```
/// # use wolf_engine_core as wolf_engine;
/// #
/// // The prelude brings in commonly needed types, and traits.
/// use wolf_engine::prelude::*;
///
/// // Start by initializing the EventLoop, and Context.
/// // In this case, we are not using any Context data, so `()` is used.
/// let (mut event_loop, mut context) = wolf_engine::init(());
///
/// // Then, you can use the EventLoop to run your game's main-loop.
/// while let Some(event) = event_loop.next_event() {
///     // Do something cool!
/// #   break;
/// }
/// ```
///
/// ## Custom Context Data
///  
/// The [`Context`] documentation has more detailed information about context data.  It's a good
/// place to start, if you're interested in customizing the engine.
///
/// ```
/// # use wolf_engine_core as wolf_engine;
/// # pub struct SomeCustomDataType {};
/// #
/// # use wolf_engine::prelude::*;
/// let (mut event_loop, mut context) = wolf_engine::init(SomeCustomDataType {});
/// ```
pub fn init<D>(data: D) -> Engine<D> {
    let event_loop = EventLoop::new();
    let context = Context::new(&event_loop, data);
    (event_loop, context)
}
