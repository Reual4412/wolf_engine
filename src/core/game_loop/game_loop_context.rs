use crate::GameLoopInfo;

/// Provides access to information and controls for the [GameLoop](crate::GameLoop).
/// 
/// # Examples
/// 
/// The only way to create a game loop context is from [GameLoop](crate::GameLoop) instance.
/// 
/// ```
/// # use wolf_engine::{GameLoopContext, FixedUpdateGameLoopBuilder, GameLoop};
/// #
/// # let game_loop = FixedUpdateGameLoopBuilder::new().build();
/// # game_loop.game_loop_info().add_tick();
/// # game_loop.game_loop_info().add_frame();
/// #
/// let game_loop_context = GameLoopContext::from_game_loop_info(&game_loop.game_loop_info());
/// 
/// assert_eq!(game_loop_context.ticks(), 1);
/// assert_eq!(game_loop_context.frames(), 1);
/// ```
pub struct GameLoopContext;

impl GameLoopContext {
    pub fn from_game_loop_info(_game_loop_info: &GameLoopInfo) -> Self {
        Self
    }
}