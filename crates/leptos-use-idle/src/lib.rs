#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Tracks whether the user is being inactive

mod use_idle;
pub use use_idle::*;

// Dependencies used through `crate::…` paths.
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
