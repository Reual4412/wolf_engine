use std::{thread, time::Duration};

use log::{debug, info};
use wolf_engine::{
    initialize_logging, Context, ContextBuilder, OptionalTransition, RenderResult, State,
    Transition, WolfEngineBuilder,
};

pub fn main() {
    initialize_logging(log::LevelFilter::Debug);

    let my_state = MyState::new("Hello, World!");

    let (context, event_loop) = ContextBuilder::new().build();
    WolfEngineBuilder::with_default_game_loop()
        .build(context)
        .run(Box::from(my_state), event_loop);
}

pub struct MyState {
    message: String,
    updates: u64,
    frames: u64,
}

impl MyState {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            updates: 0,
            frames: 0,
        }
    }
}

impl State for MyState {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        if self.frames == 10 {
            info!("Goodbye!");
            Some(Transition::Quit)
        } else {
            self.updates += 1;
            debug!("Update: {}", self.updates);
            None
        }
    }

    fn render(&mut self, _context: &mut Context) -> RenderResult {
        self.frames += 1;
        info!("{} {}", self.message, self.frames);
        thread::sleep(Duration::from_millis(32));
    }
}