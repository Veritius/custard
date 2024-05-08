use std::sync::Arc;
use rhai::plugin::*;

pub(super) fn module() -> Arc<Module> {
    let mut module = Module::new();

    FuncRegistration::new("version")
        .set_into_module(&mut module, || { env!("CARGO_PKG_VERSION") });

    return Arc::new(module);
}