#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! SSR safe `window()`

mod use_window;
pub use use_window::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_document::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_document::*;
