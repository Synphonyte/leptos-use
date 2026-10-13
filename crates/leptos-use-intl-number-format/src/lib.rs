#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [`Intl.NumberFormat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat)

mod use_intl_number_format;
pub use use_intl_number_format::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
