pub mod meta;

use rhai::Engine as RhaiEngine;
use rhai::Scope as RhaiScope;

pub struct Engine {
    pub inner: RhaiEngine,
    pub memory: RhaiScope<'static>,
}

impl Engine {
    pub fn new() -> Self {
        let mut inner = RhaiEngine::new();

        inner.register_static_module("meta", meta::module());

        Self {
            inner,
            memory: RhaiScope::new(),
        }
    }
}