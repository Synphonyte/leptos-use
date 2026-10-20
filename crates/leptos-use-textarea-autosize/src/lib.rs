#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Automatically update the height of a textarea depending on the content

mod options;
mod return_type;
mod use_textarea_autosize;
pub use options::*;
pub use return_type::*;
pub use use_textarea_autosize::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
pub(crate) use leptos_use_maybe_rw_signal::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_resize_observer::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;

#[cfg(feature = "ssr")]
use leptos_use_resize_observer as _;
