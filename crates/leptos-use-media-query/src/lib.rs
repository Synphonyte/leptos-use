#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [Media Query](https://developer.mozilla.org/en-US/docs/Web/CSS/Media_Queries/Testing_media_queries)

mod use_media_query;
pub use use_media_query::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
