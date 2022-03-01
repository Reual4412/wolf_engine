use std::mem::replace;

use crate::{
    core::{run_engine, EngineCore},
    scheduler::{FixedUpdateScheduler, Scheduler},
    Context, State, StateStack,
};

/// Provides the core functionality of the engine.
///
/// The engine is the core of, well, the engine.  It's primary job is to take and run a 
/// set of game [State] objects.  The engine uses a [StateStack] to store all active 
/// [State]s, and a [Scheduler] to control when things are run.
///
/// # Examples
///
/// If you just want to use the defaults, you can use [Engine::new()]. 
///
/// ```
/// # use wolf_engine::{Engine, EmptyState};
/// # let my_game_state = EmptyState;
/// #
/// let engine = Engine::new();
/// ```
///
/// Using [Engine::default()] does the same thing:
///
/// ```
/// # use wolf_engine::{Engine, EmptyState};
/// # let my_game_state = EmptyState;
/// #
/// let engine = Engine::default();
/// ```
///
/// If you don't want to use the default settings, the [EngineBuilder], and 
/// [ContextBuilder](crate::ContextBuilder) can be used to customize just about every 
/// aspect of the engine.
///
/// ```
/// # use wolf_engine::{Engine, EngineBuilder, ContextBuilder}
/// #
/// let context = ContextBuilder::new()
///     // Customize the context here.
///     .build();
/// let engine = EngineBuilder::new()
///     // Customize the engine here.
///     .build(context);
/// ```
///
/// You can refer to the [EngineBuilder], and [ContextBuilder](crate::ContextBuilder) 
/// documentation for specifics on each object can do.
/// 
/// Running the engine is the same, no matter if you're using the default instance, or 
/// a customized instance.  Just run [Engine::run()] and pass your games starting [State]
/// to it.
///
/// ```
/// # use wolf_engine::{Engine, EmptyState}
/// #
/// # let engine = Engine::default();
/// # let my_game_state = EmptyState;
/// #
/// engine.run(Box::from(my_game_state));
/// ```
///
/// # Engine Cores
///
/// The engine doesn't run the main loop on it's own.  Instead, it delegates the main loop
/// to an [EngineCore] function.  This helps to make the engine more modular, and 
/// customizable.  An [EngineCore] can be used to change the specific way the engine runs
/// with ease, and is primarily used to integrate with 3rd party modules that insist
/// on being control of the main loop (such as Winit.)  See [EngineCore]'s documentation 
/// for more details.
pub struct Engine {
    pub context: Context,
    pub scheduler: Box<dyn Scheduler>,
    pub state_stack: StateStack,
    core: EngineCore,
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, initial_state: Box<dyn State>) {
        self.state_stack.push(initial_state);
        let (engine, core_function) = self.extract_core_function();
        (core_function)(engine);
    }

    fn extract_core_function(mut self) -> (Engine, Box<dyn Fn(Engine)>) {
        let mut engine = replace(&mut self, Self::empty());
        let engine_core = replace(&mut engine.core, Box::from(|_| {}));
        (engine, engine_core)
    }

    fn empty() -> Self {
        Self {
            context: Context::default(),
            scheduler: Box::from(FixedUpdateScheduler::default()),
            state_stack: StateStack::new(),
            core: Box::from(|_| {}),
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        let context = Context::default();
        EngineBuilder::new().build(context)
    }
}

/// Build and customize an instance of the [Engine].
pub struct EngineBuilder {
    scheduler: Box<dyn Scheduler>,
    core: EngineCore,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self, context: Context) -> Engine {
        Engine {
            context,
            scheduler: self.scheduler,
            state_stack: StateStack::new(),
            core: self.core,
        }
    }

    pub fn with_scheduler(mut self, scheduler: Box<dyn Scheduler>) -> Self {
        self.scheduler = scheduler;
        self
    }

    pub fn with_engine_core(mut self, engine_core: EngineCore) -> Self {
        self.core = engine_core;
        self
    }
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            scheduler: Box::from(FixedUpdateScheduler::default()),
            core: Box::from(run_engine),
        }
    }
}

#[cfg(test)]
mod wolf_engine_tests {
    use crate::{MockState, Transition};

    use super::*;

    #[test]
    fn should_run_the_state() {
        let wolf_engine = Engine::default();
        let mut state = MockState::new();
        state
            .expect_update()
            .times(1..)
            .returning(|_| Some(Transition::Quit));
        state.expect_render().times(1..).returning(|_| ());

        wolf_engine.run(Box::from(state));
    }
}

#[cfg(test)]
mod engine_builder_tests {
    use std::sync::Mutex;

    use lazy_static::lazy_static;

    use super::*;
    use crate::{scheduler::MockScheduler, EmptyState};

    #[test]
    fn should_allow_custom_states() {
        let context = Context::default();
        let mut scheduler = MockScheduler::new();
        scheduler
            .expect_update()
            .times(1)
            .returning(|context, state_stack| {
                state_stack.update(context);
            });
        scheduler.expect_render().times(..).return_const(());

        EngineBuilder::new()
            .with_scheduler(Box::from(scheduler))
            .build(context)
            .run(Box::from(EmptyState));
    }

    #[test]
    fn should_set_engine_core() {
        lazy_static! {
            static ref HAS_RAN_CUSTOM_CORE: Mutex<bool> = Mutex::from(false);
        }
        let context = Context::default();
        let engine = EngineBuilder::new()
            .with_engine_core(Box::from(|_| {
                *HAS_RAN_CUSTOM_CORE.lock().unwrap() = true;
            }))
            .build(context);

        engine.run(Box::from(EmptyState));

        assert!(
            *HAS_RAN_CUSTOM_CORE.lock().unwrap(),
            "The custom engine core was not used"
        );
    }
}
