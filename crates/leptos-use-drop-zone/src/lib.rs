#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Create a zone where files can be dropped

mod use_drop_zone;
pub use use_drop_zone::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
