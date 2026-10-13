#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Tracks the visibility of an element within the viewport

mod use_element_visibility;
pub use use_element_visibility::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_element::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_intersection_observer::*;
