pub trait Context {}

pub struct Engine<C: Context> {
    context: C,
}

impl<C: Context> Engine<C> {
    pub fn new(context: C) -> Self { 
        Self { context }
    }


    pub fn context(&self) -> &C {
        &self.context
    }
}

#[cfg(test)]
mod engine_tests {
    use super::*;

    struct TestData {}

    impl TestData { 
        pub fn new() -> Self { Self {} }
    }

    impl Context for TestData {}

    #[test]
    fn should_provide_context_accessors() {
        let mut engine = Engine::new(TestData::new());

        assert_eq!(engine.context().message = "Hello, World!");
    }
}
