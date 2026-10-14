#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [`Intl.NumberFormat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat)

mod options;
mod return_type;
mod use_intl_number_format;
pub use options::*;
pub use return_type::*;
pub use use_intl_number_format::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;
pub(crate) use wasm_js_macros::*;
