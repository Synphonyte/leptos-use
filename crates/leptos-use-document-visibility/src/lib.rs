#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactively track `document.visibilityState`

mod use_document_visibility;
pub use use_document_visibility::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
