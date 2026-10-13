#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Listen for clicks outside an element

mod is;
mod on_click_outside;
pub use is::*;
pub use on_click_outside::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
