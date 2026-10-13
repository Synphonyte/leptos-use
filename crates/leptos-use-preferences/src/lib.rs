#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive prefers-color-scheme / prefers-contrast / prefers-reduced-motion for Leptos

mod use_preferred_contrast;
mod use_preferred_dark;
mod use_prefers_reduced_motion;
pub use use_preferred_contrast::*;
pub use use_preferred_dark::*;
pub use use_prefers_reduced_motion::*;

#[allow(unused_imports)]
pub(crate) mod utils {
    pub use leptos_use_header::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_header::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_media_query::*;
