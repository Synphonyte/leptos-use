#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive local / session / custom storage for Leptos

mod options;
mod use_local_storage;
mod use_session_storage;
mod use_storage;

pub use leptos_use_maybe_rw_signal::MaybeRwSignal;
pub use options::*;
pub use use_local_storage::*;
pub use use_session_storage::*;
pub use use_storage::*;

use leptos::prelude::window;
use wasm_bindgen::JsValue;

/// Local or session storage or a custom store that is a `web_sys::Storage`.
#[derive(Default)]
pub enum StorageType {
    #[default]
    Local,
    Session,
    Custom(web_sys::Storage),
}

impl StorageType {
    pub fn into_storage(self) -> Result<Option<web_sys::Storage>, JsValue> {
        match self {
            StorageType::Local => window().local_storage(),
            StorageType::Session => window().session_storage(),
            StorageType::Custom(storage) => Ok(Some(storage)),
        }
    }
}

// needed to name the filter type of `UseStorageOptions`
pub use leptos_use_filters::{
    CallFilter, DebounceOptions, FilterOptions, NoFilter, ThrottleOptions,
};

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_watch_with_options::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_window::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;

#[cfg(feature = "ssr")]
use {leptos_use_event_listener as _, leptos_use_window as _};
