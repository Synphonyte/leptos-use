#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reports changes to the dimensions of an Element's content or the border-box

mod use_resize_observer;
pub use use_resize_observer::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_supported::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
