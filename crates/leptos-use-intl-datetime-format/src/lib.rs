#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [`Intl.DateTimeFormat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat)

mod use_intl_datetime_format;
#[allow(ambiguous_glob_reexports)]
pub use use_intl_datetime_format::*;

#[allow(unused_imports)]
pub(crate) mod utils {
    pub use sendwrap_fn::*;
    pub use wasm_js_macros::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
