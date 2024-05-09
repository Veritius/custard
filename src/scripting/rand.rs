use std::sync::Arc;
use fastrand::*;
use rhai::{plugin::*, CustomType, TypeBuilder};

pub(super) fn setup(engine: &mut Engine) {
    let mut module = Module::new();

    engine.register_type::<RhaiRng>();

    FuncRegistration::new("new")
        .set_into_module(&mut module, |seed: u64| RhaiRng::new(seed));

    engine.register_static_module("rand", Arc::new(module));
}

#[derive(Clone)]
pub struct RhaiRng(pub Rng);

impl RhaiRng {
    pub fn new(seed: u64) -> Self {
        Self(Rng::with_seed(seed))
    }
}

impl CustomType for RhaiRng {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
        .with_name("Rng")
        .with_fn("fork", |s: &Self| s.clone())
        .with_fn("bool", |s: &mut Self| s.0.bool())
        .with_fn("letter", |s: &mut Self| s.0.lowercase())
        .with_fn("digit", |s: &mut Self| s.0.digit(10))
        .with_fn("float", |s: &mut Self| s.0.f64())
        .with_fn("int", |s: &mut Self| {
            #[cfg(target_pointer_width="32")]
            return s.0.i32(..);

            #[cfg(target_pointer_width="64")]
            return s.0.i64(..);
        });
    }
}