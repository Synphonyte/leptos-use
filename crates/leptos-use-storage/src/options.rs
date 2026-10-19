use crate::{MaybeRwSignal, NoFilter};
use codee::CodecError;
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
/// Options for [`fn@crate::use_storage_with_options`].
///
/// The filter is a type parameter so that the debounce and throttle code is only compiled into
/// your app if you use it.
pub struct UseStorageOptions<T, E, D, F = NoFilter>
where
    T: Send + Sync + 'static,
{
    // Callback for when an error occurs
    pub(crate) on_error: Arc<dyn Fn(UseStorageError<E, D>) + Send + Sync>,
    // Whether to continuously listen to changes from browser storage
    pub(crate) listen_to_storage_changes: bool,
    // Initial value to use when the storage key is not set
    pub(crate) initial_value: MaybeRwSignal<T>,
    // Debounce or throttle the writing to storage whenever the value changes
    pub(crate) filter: F,
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
            filter: NoFilter,
            delay_during_hydration: false,
        }
    }
}

impl<T, E, D, F> UseStorageOptions<T, E, D, F>
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

    /// Whether to continuously listen to changes from browser storage. Defaults to `true`.
    pub fn listen_to_storage_changes(self, listen_to_storage_changes: bool) -> Self {
        Self {
            listen_to_storage_changes,
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

    /// Debounce or throttle the writing to storage whenever the value changes with a
    /// [`FilterOptions`](crate::FilterOptions). Defaults to no filter.
    pub fn filter<NewF>(self, filter: NewF) -> UseStorageOptions<T, E, D, NewF> {
        UseStorageOptions {
            on_error: self.on_error,
            listen_to_storage_changes: self.listen_to_storage_changes,
            initial_value: self.initial_value,
            filter,
            delay_during_hydration: self.delay_during_hydration,
        }
    }

    /// Delays the reading of the value from storage by one animation frame during hydration.
    /// This ensures that during hydration the value is the initial value just like it is on the server
    /// which helps prevent hydration errors. Defaults to `false`.
    pub fn delay_during_hydration(self, delay_during_hydration: bool) -> Self {
        Self {
            delay_during_hydration,
            ..self
        }
    }
}
