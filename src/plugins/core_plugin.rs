use crate::contexts::*;
use crate::*;

/// Provides core functionality that **must** be loaded in order for the engine to work.
pub(crate) struct CorePlugin;

impl Plugin for CorePlugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult {
        Ok(engine_builder
            .with_subcontext(EngineContext::new())
            .with_subcontext(SchedulerContext::new()))
    }
}
