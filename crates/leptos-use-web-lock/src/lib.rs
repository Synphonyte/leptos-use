#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Rustified [Web Locks API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Locks_API)

#[cfg(web_sys_unstable_apis)]
mod options;
#[cfg(web_sys_unstable_apis)]
mod use_web_lock;
#[cfg(web_sys_unstable_apis)]
pub use options::*;
#[cfg(web_sys_unstable_apis)]
pub use use_web_lock::*;

// Dependencies used through `crate::…` paths.
#[cfg(all(web_sys_unstable_apis, not(feature = "ssr")))]
pub(crate) use wasm_js_macros::*;

#[cfg(not(web_sys_unstable_apis))]
use {
    default_struct_builder as _, thiserror as _, wasm_bindgen as _, wasm_bindgen_futures as _,
    web_sys as _,
};
#[cfg(any(feature = "ssr", not(web_sys_unstable_apis)))]
use {leptos as _, wasm_js_macros as _};
