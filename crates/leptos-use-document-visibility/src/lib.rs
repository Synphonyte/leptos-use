#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactively track `document.visibilityState`

mod use_document_visibility;
pub use use_document_visibility::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_event_listener::*;
