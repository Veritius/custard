pub mod meta;
pub mod rand;

use rhai::Engine as RhaiEngine;

pub struct Engine {
    pub inner: RhaiEngine,
}

impl Engine {
    pub fn new() -> Self {
        let mut engine = RhaiEngine::new();

        meta::setup(&mut engine);
        rand::setup(&mut engine);

        Self {
            inner: engine,
        }
    }
}