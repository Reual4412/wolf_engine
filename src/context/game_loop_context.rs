use std::sync::{Arc, Mutex};

use crate::game_loop::{Frames, Ticks};

/// Provides access to information and controls for the [GameLoop](crate::game_loop::GameLoop).
///
/// # Examples
///
/// The GameLoopContext can be created directly using the new method.
///
/// ```
/// # use wolf_engine::context::GameLoopContext;
/// #
/// let game_loop_context = GameLoopContext::new();
/// ```
///
/// Once created, the GameLoopContext exposes information about the [GameLoop](crate::game_loop::GameLoop).
///
/// ```
/// # use wolf_engine::context::GameLoopContext;
/// #
/// # let game_loop_context = GameLoopContext::new();
/// #
/// game_loop_context.ticks();
/// game_loop_context.frames();
/// ```
///
/// Tick and frame information can be added to the context.  
///
/// **Note:** These method are only intended for the [GameLoop](crate::game_loop::GameLoop) and other parts of
/// the engine. If you are not providing a custom game loop, you **should not** touch these.
///
/// ```
/// # use wolf_engine::context::GameLoopContext;
/// #
/// # let game_loop_context = GameLoopContext::new();
/// #
/// # assert_eq!(game_loop_context.ticks(), 0, "There should be 0 ticks before add_tick is called");
/// # assert_eq!(game_loop_context.frames(), 0, "There should be 0 frames before add_tick is called");
/// #
/// game_loop_context.add_tick();
/// game_loop_context.add_frame();
/// #
/// # game_loop_context.ticks();
/// # game_loop_context.frames();
/// #
/// # assert_eq!(game_loop_context.ticks(), 1, "1 tick should have been added");
/// # assert_eq!(game_loop_context.frames(), 1, "1 frame should have been added");
/// ```
pub struct GameLoopContext {
    ticks: Arc<Mutex<Ticks>>,
    frames: Arc<Mutex<Frames>>,
}

impl GameLoopContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_tick(&self) {
        *self.ticks.lock().unwrap() += 1;
    }

    pub fn ticks(&self) -> Ticks {
        *self.ticks.lock().unwrap()
    }

    pub fn add_frame(&self) {
        *self.frames.lock().unwrap() += 1;
    }

    pub fn frames(&self) -> Frames {
        *self.frames.lock().unwrap()
    }
}

impl Default for GameLoopContext {
    fn default() -> Self {
        Self {
            ticks: Arc::from(Mutex::from(0)),
            frames: Arc::from(Mutex::from(0)),
        }
    }
}