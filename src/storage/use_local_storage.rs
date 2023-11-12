use super::{use_storage, Codec, StorageType, UseStorageOptions};
use leptos::signal_prelude::*;

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
    T: Clone + Default + PartialEq,
    C: Codec<T> + Default,
{
    use_storage_with_options(
        StorageType::Local,
        key,
        UseStorageOptions::<T, C>::default(),
    )
}

/// Accepts [`UseStorageOptions`]. See [`use_local_storage`] for details.
pub fn use_local_storage_with_options<T, C>(
    key: impl AsRef<str>,
    options: UseStorageOptions<T, C>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() + Clone)
where
    T: Clone + PartialEq,
    C: Codec<T>,
{
    use_storage_with_options(StorageType::Local, key, options)
}
