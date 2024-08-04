use super::{use_storage_with_options, StorageType, UseStorageOptions};
use codee::{Decoder, Encoder};
use leptos::prelude::wrappers::read::Signal;
use leptos::prelude::*;

/// Reactive [LocalStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage).
///
/// LocalStorage stores data in the browser with no expiration time. Access is given to all pages from the same origin (e.g., all pages from "https://example.com" share the same origin). While data doesn't expire the user can view, modify and delete all data stored. Browsers allow 5MB of data to be stored.
///
/// This is contrast to [`use_session_storage`] which clears data when the page session ends and is not shared.
///
/// ## Usage
/// See [`use_storage`] for more details on how to use.
pub fn use_local_storage<T, C>(
    key: impl AsRef<str>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() + Clone)
where
    T: Clone + Default + PartialEq + Send + Sync + 'static,
    C: Encoder<T, Encoded = String> + Decoder<T, Encoded = str>,
{
    use_storage_with_options::<T, C>(
        StorageType::Local,
        key,
        UseStorageOptions::<T, <C as Encoder<T>>::Error, <C as Decoder<T>>::Error>::default(),
    )
}

/// Accepts [`UseStorageOptions`]. See [`use_local_storage`] for details.
pub fn use_local_storage_with_options<T, C>(
    key: impl AsRef<str>,
    options: UseStorageOptions<T, <C as Encoder<T>>::Error, <C as Decoder<T>>::Error>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() + Clone)
where
    T: Clone + PartialEq + Send + Sync,
    C: Encoder<T, Encoded = String> + Decoder<T, Encoded = str>,
{
    use_storage_with_options::<T, C>(StorageType::Local, key, options)
}
