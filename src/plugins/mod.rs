use crate::{EngineBuilder, Plugin, contexts::{SchedulerContext, EventContext}, event::Event};

pub(crate) struct CorePlugin; 

impl Plugin for CorePlugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> EngineBuilder {
        engine_builder
            .with_subcontext(SchedulerContext::new())
            .with_subcontext(EventContext::<Event>::default())
    }
}
