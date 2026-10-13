#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [IntersectionObserver](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver)

mod use_intersection_observer;
pub use use_intersection_observer::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_element::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_watch_with_options::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
