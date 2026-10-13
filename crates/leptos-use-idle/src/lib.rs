#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Tracks whether the user is being inactive

mod use_idle;
pub use use_idle::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_document::*;
    pub use leptos_use_now::*;
}

#[allow(unused_imports)]
pub(crate) mod utils {
    pub use leptos_use_filters::*;
    pub use sendwrap_fn::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_document::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_filters::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_now::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
