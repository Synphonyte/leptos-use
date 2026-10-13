#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive size of an HTML element

mod use_element_size;
pub use use_element_size::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_core::*;
    pub use leptos_use_element::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_core::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_resize_observer::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_watch_with_options::*;
