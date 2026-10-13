#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive local / session / custom storage for Leptos

mod storage;
pub use storage::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_maybe_rw_signal::*;
}

#[allow(unused_imports)]
pub(crate) mod utils {
    pub use leptos_use_filters::*;
    pub use sendwrap_fn::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_filters::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_maybe_rw_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_watch_with_options::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
