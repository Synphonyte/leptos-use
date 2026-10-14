#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Debounce/throttle/pausable filters used by the leptos-use crates

macro_rules! ArcFilterFn {
    ($R:ident) => {
        Arc<dyn Fn(Arc<dyn Fn() -> $R>) -> Arc<Mutex<Option<$R>>>>
    }
}

mod filters;
mod options;
mod pausable;
pub use filters::*;
pub use options::*;
pub use pausable::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_now::*;
pub(crate) use sendwrap_fn::*;
