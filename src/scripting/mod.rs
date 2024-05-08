use rhai::Engine as RhaiEngine;
use rhai::Scope as RhaiScope;

pub struct Engine {
    pub inner: RhaiEngine,
    pub memory: RhaiScope<'static>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            inner: RhaiEngine::new(),
            memory: RhaiScope::new(),
        }
    }
}