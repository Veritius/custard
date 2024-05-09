pub mod meta;
pub mod rand;

use rhai::Engine as RhaiEngine;
use rhai::Scope as RhaiScope;

pub struct Engine {
    pub inner: RhaiEngine,
    pub memory: RhaiScope<'static>,
}

impl Engine {
    pub fn new() -> Self {
        let mut engine = RhaiEngine::new();

        meta::setup(&mut engine);
        rand::setup(&mut engine);

        Self {
            inner: engine,
            memory: RhaiScope::new(),
        }
    }
}