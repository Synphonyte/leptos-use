#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [IntersectionObserver](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver)

mod options;
mod return_type;
mod use_intersection_observer;
pub use options::*;
pub use return_type::*;
pub use use_intersection_observer::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_watch_with_options::*;
pub(crate) use sendwrap_fn::*;
