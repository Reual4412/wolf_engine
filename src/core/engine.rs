use crate::{FixedUpdateGameLoop, FixedUpdateGameLoopBuilder, GameLoop};


/// Provides access to all engine state.
/// 
/// # Examples
/// 
/// Using the default game loop.
/// 
/// ```
/// # use wolf_engine::WolfEngine;
/// # use wolf_engine::FixedUpdateGameLoop
/// 
/// let engine: WolfEngine<FixedUpdateGameLoop> = WolfEngine::with_default_game_loop()
///     .build();
/// ```
pub struct WolfEngine<Loop: GameLoop> {
    game_loop: Loop
}

pub struct WolfEngineBuilder<Loop: GameLoop> {
    engine: WolfEngine<Loop>
}

impl WolfEngineBuilder<FixedUpdateGameLoop> {
    pub fn with_default_game_loop() -> Self {
        let game_loop = FixedUpdateGameLoop::default();
        let engine = WolfEngine {
            game_loop,
        };
        Self {
            engine
        }
    }
}