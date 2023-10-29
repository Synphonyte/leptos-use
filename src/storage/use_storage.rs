use crate::{
    core::{MaybeRwSignal, StorageType},
    use_event_listener, use_window,
    utils::FilterOptions,
    watch_with_options, WatchOptions,
};
use cfg_if::cfg_if;
use leptos::*;
use std::rc::Rc;
use thiserror::Error;
use wasm_bindgen::JsValue;

const INTERNAL_STORAGE_EVENT: &str = "leptos-use-storage";

/// A codec for encoding and decoding values to and from UTF-16 strings. These strings are then stored in browser storage.
pub trait Codec<T>: Clone + 'static {
    /// The error type returned when encoding or decoding fails.
    type Error;
    /// Encodes a value to a UTF-16 string.
    fn encode(&self, val: &T) -> Result<String, Self::Error>;
    /// Decodes a UTF-16 string to a value. Should be able to decode any string encoded by [`encode`].
    fn decode(&self, str: String) -> Result<T, Self::Error>;
}

/// Options for use with [`use_local_storage_with_options`], [`use_session_storage_with_options`] and [`use_storage_with_options`].
pub struct UseStorageOptions<T: 'static, C: Codec<T>> {
    codec: C,
    on_error: Rc<dyn Fn(UseStorageError<C::Error>)>,
    listen_to_storage_changes: bool,
    default_value: MaybeRwSignal<T>,
    filter: FilterOptions,
}

/// Session handling errors returned by [`use_storage`].
#[derive(Error, Debug)]
pub enum UseStorageError<Err> {
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
    ItemCodecError(Err),
}

/// Reactive [LocalStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage).
///
/// LocalStorage stores data in the browser with no expiration time. Access is given to all pages from the same origin (e.g., all pages from "https://example.com" share the same origin). While data doesn't expire the user can view, modify and delete all data stored. Browsers allow 5MB of data to be stored.
///
/// This is contrast to [`use_session_storage`] which clears data when the page session ends and is not shared.
///
/// See [`use_storage_with_options`] for more details on how to use.
pub fn use_local_storage<T, C>(
    key: impl AsRef<str>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() -> () + Clone)
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
) -> (Signal<T>, WriteSignal<T>, impl Fn() -> () + Clone)
where
    T: Clone + PartialEq,
    C: Codec<T>,
{
    use_storage_with_options(StorageType::Local, key, options)
}

/// Reactive [SessionStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage).
///
/// SessionStorages stores data in the browser that is deleted when the page session ends. A page session ends when the browser closes the tab. Data is not shared between pages. While data doesn't expire the user can view, modify and delete all data stored. Browsers allow 5MB of data to be stored.
///
/// Use [`use_local_storage`] to store data that is shared amongst all pages with the same origin and persists between page sessions.
///
/// See [`use_storage_with_options`] for more details on how to use.
pub fn use_session_storage<T, C>(
    key: impl AsRef<str>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() -> () + Clone)
where
    T: Clone + Default + PartialEq,
    C: Codec<T> + Default,
{
    use_storage_with_options(
        StorageType::Session,
        key,
        UseStorageOptions::<T, C>::default(),
    )
}

/// Accepts [`UseStorageOptions`]. See [`use_session_storage`] for details.
pub fn use_session_storage_with_options<T, C>(
    key: impl AsRef<str>,
    options: UseStorageOptions<T, C>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() -> () + Clone)
where
    T: Clone + PartialEq,
    C: Codec<T>,
{
    use_storage_with_options(StorageType::Session, key, options)
}

/// Reactive [Storage](https://developer.mozilla.org/en-US/docs/Web/API/Storage).
///
/// * [See a demo](https://leptos-use.rs/storage/use_storage.html)
/// * [See a full example](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_storage)
///
/// ## Usage
///
/// Pass a [`StorageType`] to determine the kind of key-value browser storage to use. The specified key is where data is stored. All values are stored as UTF-16 strings which is then encoded and decoded via the given [`Codec`].Finally, see [`UseStorageOptions`] to see how behaviour can be further customised.
///
/// Returns a triplet `(read_signal, write_signal, delete_from_storage_fn)`.
///
/// Signals work as expected and can be used to read and write to storage. The `delete_from_storage_fn` can be called to delete the item from storage. Once deleted the signals will revert back to the default value.
///
/// ## Example
///
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage_with_options, UseStorageOptions, StringCodec, JsonCodec, ProstCodec};
/// # use serde::{Deserialize, Serialize};
/// #
/// # pub fn Demo() -> impl IntoView {
/// // Binds a struct:
/// let (state, set_state, _) = use_local_storage::<MyState, JsonCodec>("my-state");
///
/// // Binds a bool, stored as a string:
/// let (flag, set_flag, remove_flag) = use_session_storage::<bool, StringCodec>("my-flag");
///
/// // Binds a number, stored as a string:
/// let (count, set_count, _) = use_session_storage::<i32, StringCodec>("my-count");
/// // Binds a number, stored in JSON:
/// let (count, set_count, _) = use_session_storage::<i32, JsonCodec>("my-count-kept-in-js");
///
/// // Bind string with SessionStorage stored in ProtoBuf format:
/// let (id, set_id, _) = use_storage_with_options::<String, ProstCodec>(
///     StorageType::Session,
///     "my-id",
///     UseStorageOptions::prost_codec(),
/// );
/// #    view! { }
/// # }
///
/// // Data stored in JSON must implement Serialize, Deserialize:
/// #[derive(Serialize, Deserialize, Clone, PartialEq)]
/// pub struct MyState {
///     pub hello: String,
///     pub greeting: String,
/// }
///
/// // Default can be used to implement intial or deleted values.
/// // You can also use a signal via UseStorageOptions::default_value`
/// impl Default for MyState {
///     fn default() -> Self {
///         Self {
///             hello: "hi".to_string(),
///             greeting: "Hello".to_string()
///         }
///     }
/// }
/// ```
pub fn use_storage_with_options<T, C>(
    storage_type: StorageType,
    key: impl AsRef<str>,
    options: UseStorageOptions<T, C>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() -> () + Clone)
where
    T: Clone + PartialEq,
    C: Codec<T>,
{
    /*
    cfg_if! { if #[cfg(feature = "ssr")] {
        let (data, set_data) = create_signal(None);
        let set_value = move |value: Option<T>| {
            set_data.set(value);
        };
        let value = create_memo(move |_| data.get().unwrap_or_default());
        return (value, set_value, || ());
    } else {
        // Continue
    }}*/

    let UseStorageOptions {
        codec,
        on_error,
        listen_to_storage_changes,
        default_value,
        filter,
    } = options;

    // Get storage API
    let storage = storage_type
        .into_storage()
        .map_err(UseStorageError::StorageNotAvailable)
        .and_then(|s| s.ok_or(UseStorageError::StorageReturnedNone));
    let storage = handle_error(&on_error, storage);

    // Schedules a storage event microtask. Uses a queue to avoid re-entering the runtime
    let dispatch_storage_event = {
        let key = key.as_ref().to_owned();
        let on_error = on_error.to_owned();
        move || {
            let key = key.to_owned();
            let on_error = on_error.to_owned();
            queue_microtask(move || {
                // Note: we cannot construct a full StorageEvent so we _must_ rely on a custom event
                let mut custom = web_sys::CustomEventInit::new();
                custom.detail(&JsValue::from_str(&key));
                let result = window()
                    .dispatch_event(
                        &web_sys::CustomEvent::new_with_event_init_dict(
                            INTERNAL_STORAGE_EVENT,
                            &custom,
                        )
                        .expect("failed to create custom storage event"),
                    )
                    .map_err(UseStorageError::NotifyItemChangedFailed);
                let _ = handle_error(&on_error, result);
            })
        }
    };

    // Fires when storage needs to be updated
    let notify = create_trigger();

    // Keeps track of how many times we've been notified. Does not increment for calls to set_data
    let notify_id = create_memo::<usize>(move |prev| {
        notify.track();
        prev.map(|prev| prev + 1).unwrap_or_default()
    });

    // Fetch from storage and falls back to the default (possibly a signal) if deleted
    let fetcher = {
        let storage = storage.to_owned();
        let codec = codec.to_owned();
        let key = key.as_ref().to_owned();
        let on_error = on_error.to_owned();
        let (default, _) = default_value.into_signal();
        create_memo(move |_| {
            notify.track();
            storage
                .to_owned()
                .and_then(|storage| {
                    // Get directly from storage
                    let result = storage
                        .get_item(&key)
                        .map_err(UseStorageError::GetItemFailed);
                    handle_error(&on_error, result)
                })
                .unwrap_or_default() // Drop handled Err(())
                .map(|encoded| {
                    // Decode item
                    let result = codec
                        .decode(encoded)
                        .map_err(UseStorageError::ItemCodecError);
                    handle_error(&on_error, result)
                })
                .transpose()
                .unwrap_or_default() // Drop handled Err(())
                // Fallback to default
                .unwrap_or_else(move || default.get())
        })
    };

    // Create mutable data signal from our fetcher
    let (data, set_data) = MaybeRwSignal::<T>::from(fetcher).into_signal();
    let data = create_memo(move |_| data.get());

    // Set storage value on data change
    {
        let storage = storage.to_owned();
        let codec = codec.to_owned();
        let key = key.as_ref().to_owned();
        let on_error = on_error.to_owned();
        let dispatch_storage_event = dispatch_storage_event.to_owned();
        let _ = watch_with_options(
            move || (notify_id.get(), data.get()),
            move |(id, value), prev, _| {
                // Skip setting storage on changes from external events. The ID will change on external events.
                if prev.map(|(prev_id, _)| *prev_id != *id).unwrap_or_default() {
                    return;
                }

                if let Ok(storage) = &storage {
                    // Encode value
                    let result = codec
                        .encode(value)
                        .map_err(UseStorageError::ItemCodecError)
                        .and_then(|enc_value| {
                            // Set storage -- sends a global event
                            storage
                                .set_item(&key, &enc_value)
                                .map_err(UseStorageError::SetItemFailed)
                        });
                    let result = handle_error(&on_error, result);
                    // Send internal storage event
                    if result.is_ok() {
                        dispatch_storage_event();
                    }
                }
            },
            WatchOptions::default().filter(filter),
        );
    };

    if listen_to_storage_changes {
        let check_key = key.as_ref().to_owned();
        // Listen to global storage events
        let _ = use_event_listener(use_window(), leptos::ev::storage, move |ev| {
            let ev_key = ev.key();
            // Key matches or all keys deleted (None)
            if ev_key == Some(check_key.clone()) || ev_key.is_none() {
                notify.notify()
            }
        });
        // Listen to internal storage events
        let check_key = key.as_ref().to_owned();
        let _ = use_event_listener(
            use_window(),
            ev::Custom::new(INTERNAL_STORAGE_EVENT),
            move |ev: web_sys::CustomEvent| {
                if Some(check_key.clone()) == ev.detail().as_string() {
                    notify.notify()
                }
            },
        );
    };

    // Remove from storage fn
    let remove = {
        let key = key.as_ref().to_owned();
        move || {
            let _ = storage.as_ref().map(|storage| {
                // Delete directly from storage
                let result = storage
                    .remove_item(&key)
                    .map_err(UseStorageError::RemoveItemFailed);
                let _ = handle_error(&on_error, result);
                notify.notify();
                dispatch_storage_event();
            });
        }
    };

    (data.into(), set_data, remove)
}

/// Calls the on_error callback with the given error. Removes the error from the Result to avoid double error handling.
fn handle_error<T, Err>(
    on_error: &Rc<dyn Fn(UseStorageError<Err>)>,
    result: Result<T, UseStorageError<Err>>,
) -> Result<T, ()> {
    result.or_else(|err| Err((on_error)(err)))
}

impl<T: Default, C: Codec<T> + Default> Default for UseStorageOptions<T, C> {
    fn default() -> Self {
        Self {
            codec: C::default(),
            on_error: Rc::new(|_err| ()),
            listen_to_storage_changes: true,
            default_value: MaybeRwSignal::default(),
            filter: FilterOptions::default(),
        }
    }
}

impl<T: Default, C: Codec<T>> UseStorageOptions<T, C> {
    /// Optional callback whenever an error occurs.
    pub fn on_error(self, on_error: impl Fn(UseStorageError<C::Error>) + 'static) -> Self {
        Self {
            on_error: Rc::new(on_error),
            ..self
        }
    }

    /// Listen to changes to this storage key from browser and page events. Defaults to true.
    pub fn listen_to_storage_changes(self, listen_to_storage_changes: bool) -> Self {
        Self {
            listen_to_storage_changes,
            ..self
        }
    }

    /// Default value to use when the storage key is not set. Accepts a signal.
    pub fn default_value(self, values: impl Into<MaybeRwSignal<T>>) -> Self {
        Self {
            default_value: values.into(),
            ..self
        }
    }

    /// Debounce or throttle the writing to storage whenever the value changes.
    pub fn filter(self, filter: impl Into<FilterOptions>) -> Self {
        Self {
            filter: filter.into(),
            ..self
        }
    }
}
