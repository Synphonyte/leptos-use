#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Manipulate CSS variables

mod use_css_var;
pub use use_css_var::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
pub(crate) use leptos_use_watch_with_options::*;
