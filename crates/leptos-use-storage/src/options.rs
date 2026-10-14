use crate::{FilterOptions, MaybeRwSignal};
use codee::CodecError;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::sync::Arc;
use thiserror::Error;
use wasm_bindgen::JsValue;

/// Session handling errors returned by [`use_storage_with_options`](crate::use_storage_with_options).
#[derive(Error, Debug)]
pub enum UseStorageError<E, D> {
    #[error("storage not available")]
    StorageNotAvailable(JsValue),
    #[error("storage not returned from window")]
    StorageReturnedNone,
    #[error("failed to get item")]
    GetItemFailed(JsValue),
    #[error("failed to set item")]
    SetItemFailed(JsValue),
    #[error("failed to delete item")]
    RemoveItemFailed(JsValue),
    #[error("failed to notify item changed")]
    NotifyItemChangedFailed(JsValue),
    #[error("failed to encode / decode item value")]
    ItemCodecError(CodecError<E, D>),
}

/// Options for use with [`fn@crate::use_local_storage_with_options`], [`fn@crate::use_session_storage_with_options`] and [`use_storage_with_options`](crate::use_storage_with_options).
#[derive(DefaultBuilder)]
pub struct UseStorageOptions<T, E, D>
where
    T: Send + Sync + 'static,
{
    // Callback for when an error occurs
    #[builder(skip)]
    pub(crate) on_error: Arc<dyn Fn(UseStorageError<E, D>) + Send + Sync>,
    // Whether to continuously listen to changes from browser storage
    pub(crate) listen_to_storage_changes: bool,
    // Initial value to use when the storage key is not set
    #[builder(skip)]
    pub(crate) initial_value: MaybeRwSignal<T>,
    // Debounce or throttle the writing to storage whenever the value changes
    #[builder(into)]
    pub(crate) filter: FilterOptions,
    /// Delays the reading of the value from storage by one animation frame during hydration.
    /// This ensures that during hydration the value is the initial value just like it is on the server
    /// which helps prevent hydration errors. Defaults to `false`.
    pub(crate) delay_during_hydration: bool,
}

impl<T: Default, E, D> Default for UseStorageOptions<T, E, D>
where
    T: Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            on_error: Arc::new(|_err| ()),
            listen_to_storage_changes: true,
            initial_value: MaybeRwSignal::default(),
            filter: FilterOptions::default(),
            delay_during_hydration: false,
        }
    }
}

impl<T: Default, E, D> UseStorageOptions<T, E, D>
where
    T: Send + Sync + 'static,
{
    /// Optional callback whenever an error occurs.
    pub fn on_error(
        self,
        on_error: impl Fn(UseStorageError<E, D>) + Send + Sync + 'static,
    ) -> Self {
        Self {
            on_error: Arc::new(on_error),
            ..self
        }
    }

    /// Initial value to use when the storage key is not set. Note that this value is read once on creation of the storage hook and not updated again. Accepts a signal and defaults to `T::default()`.
    pub fn initial_value(self, initial: impl Into<MaybeRwSignal<T>>) -> Self {
        Self {
            initial_value: initial.into(),
            ..self
        }
    }
}
