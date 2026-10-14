#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Create a zone where files can be dropped

mod options;
mod return_type;
mod use_drop_zone;
pub use options::*;
pub use return_type::*;
pub use use_drop_zone::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
