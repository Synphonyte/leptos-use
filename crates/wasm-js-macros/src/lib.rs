#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Macros for common JavaScript operations (`in`, property get/set, futures) with wasm-bindgen

mod js;
mod js_value_from_to_string;

#[doc(hidden)]
pub mod __private {
    pub use js_sys;
    pub use wasm_bindgen;
    pub use wasm_bindgen_futures;
}
