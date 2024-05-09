use std::sync::Arc;
use rhai::plugin::*;

pub(super) fn setup(engine: &mut Engine) {
    let mut module = Module::new();

    FuncRegistration::new("version")
        .set_into_module(&mut module, || { env!("CARGO_PKG_VERSION") });

    engine.register_static_module("meta", Arc::new(module));
}