#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive prefers-color-scheme / prefers-contrast / prefers-reduced-motion for Leptos

mod options;
mod use_preferred_contrast;
mod use_preferred_dark;
mod use_prefers_reduced_motion;
pub use options::*;
pub use use_preferred_contrast::*;
pub use use_preferred_dark::*;
pub use use_prefers_reduced_motion::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_header::*;
pub(crate) use leptos_use_media_query::*;
