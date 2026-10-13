#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Macros for common JavaScript operations (`in`, property get/set, futures) with wasm-bindgen

mod js;
mod js_value_from_to_string;
#[allow(unused_imports)]
pub use js::*;
#[allow(unused_imports)]
pub use js_value_from_to_string::*;
