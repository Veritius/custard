use std::sync::Arc;
use rhai::plugin::*;

pub(super) fn module() -> Arc<Module> {
    let mut module = Module::new();

    FuncRegistration::new("delta_secs")
        .with_volatility(true)
        .set_into_module(&mut module, || { todo!() });

    return Arc::new(module);
}