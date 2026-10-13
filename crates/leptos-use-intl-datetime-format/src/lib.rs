#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [`Intl.DateTimeFormat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat)

mod use_intl_datetime_format;
#[allow(ambiguous_glob_reexports)]
pub use use_intl_datetime_format::*;

// Dependencies used through `crate::…` paths.
pub(crate) use sendwrap_fn::*;
pub(crate) use wasm_js_macros::*;
