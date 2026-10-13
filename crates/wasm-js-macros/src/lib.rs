#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Macros for common JavaScript operations (`in`, property get/set, futures) with wasm-bindgen

#[allow(unused_imports)]
pub(crate) mod utils {
    pub(crate) mod js;
    pub(crate) mod js_value_from_to_string;
    pub use js::*;
    pub use js_value_from_to_string::*;
}
#[allow(unused_imports)]
pub use utils::js::*;
#[allow(unused_imports)]
pub use utils::js_value_from_to_string::*;
