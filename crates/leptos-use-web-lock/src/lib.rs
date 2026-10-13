#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Rustified [Web Locks API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Locks_API)

#[cfg(web_sys_unstable_apis)]
mod use_web_lock;
#[cfg(web_sys_unstable_apis)]
pub use use_web_lock::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
