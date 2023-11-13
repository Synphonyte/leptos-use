use crate::storage::StringCodec;
use crate::{
    core::{MaybeRwSignal, StorageType},
    utils::FilterOptions,
};
use cfg_if::cfg_if;
use leptos::*;
use std::rc::Rc;
use thiserror::Error;
use wasm_bindgen::JsValue;

const INTERNAL_STORAGE_EVENT: &str = "leptos-use-storage";

/// Reactive [Storage](https://developer.mozilla.org/en-US/docs/Web/API/Storage).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_storage)
///
/// ## Usage
///
/// Pass a [`StorageType`] to determine the kind of key-value browser storage to use. The specified key is where data is stored. All values are stored as UTF-16 strings which is then encoded and decoded via the given [`Codec`]. This value is synced with other calls using the same key on the smae page and across tabs for local storage. See [`UseStorageOptions`] to see how behaviour can be further customised.
///
/// See [`Codec`] for more details on how to handle versioning--dealing with data that can outlast your code.
///
/// Returns a triplet `(read_signal, write_signal, delete_from_storage_fn)`.
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
///     UseStorageOptions::default(),
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
#[inline(always)]
pub fn use_storage(
    storage_type: StorageType,
    key: impl AsRef<str>,
) -> (Signal<String>, WriteSignal<String>, impl Fn() + Clone) {
    use_storage_with_options::<String, StringCodec>(storage_type, key, UseStorageOptions::default())
}

/// Version of [`use_storage`] that accepts [`UseStorageOptions`].
pub fn use_storage_with_options<T, C>(
    storage_type: StorageType,
    key: impl AsRef<str>,
    options: UseStorageOptions<T, C>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() + Clone)
where
    T: Clone + PartialEq,
    C: Codec<T>,
{
    let UseStorageOptions {
        codec,
        on_error,
        listen_to_storage_changes,
        initial_value,
        filter,
    } = options;

    let (data, set_data) = initial_value.into_signal();
    let default = data.get_untracked();

    cfg_if! { if #[cfg(feature = "ssr")] {
        let _ = codec;
        let _ = on_error;
        let _ = listen_to_storage_changes;
        let _ = filter;
        let _ = storage_type;
        let _ = key;
        let _ = INTERNAL_STORAGE_EVENT;


        let remove = move || {
            set_data.set(default.clone());
        };

        (data.into(), set_data, remove)
    } else {
        use crate::{use_event_listener, use_window, watch_with_options, WatchOptions};

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

        // Fetches direct from browser storage and fills set_data if changed (memo)
        let fetch_from_storage = {
            let storage = storage.to_owned();
            let codec = codec.to_owned();
            let key = key.as_ref().to_owned();
            let on_error = on_error.to_owned();
            move || {
                let fetched = storage
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
                    .unwrap_or_default(); // Drop handled Err(())

                match fetched {
                    Some(value) => {
                        // Replace data if changed
                        if value != data.get_untracked() {
                            set_data.set(value)
                        }
                    }

                    // Revert to default
                    None => set_data.set(default.clone()),
                };
            }
        };

        // Fetch initial value
        fetch_from_storage();

        // Fires when storage needs to be fetched
        let notify = create_trigger();

        // Refetch from storage. Keeps track of how many times we've been notified. Does not increment for calls to set_data
        let notify_id = create_memo::<usize>(move |prev| {
            notify.track();
            match prev {
                None => 1, // Avoid async fetch of initial value
                Some(prev) => {
                    fetch_from_storage();
                    prev + 1
                }
            }
        });

        // Set item on internal (non-event) page changes to the data signal
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
        }

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

        (data, set_data, remove)
    }}
}

/// Session handling errors returned by [`use_storage_with_options`].
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

/// Options for use with [`use_local_storage_with_options`], [`use_session_storage_with_options`] and [`use_storage_with_options`].
pub struct UseStorageOptions<T: 'static, C: Codec<T>> {
    // Translates to and from UTF-16 strings
    codec: C,
    // Callback for when an error occurs
    on_error: Rc<dyn Fn(UseStorageError<C::Error>)>,
    // Whether to continuously listen to changes from browser storage
    listen_to_storage_changes: bool,
    // Initial value to use when the storage key is not set
    initial_value: MaybeRwSignal<T>,
    // Debounce or throttle the writing to storage whenever the value changes
    filter: FilterOptions,
}

/// A codec for encoding and decoding values to and from UTF-16 strings. These strings are intended to be stored in browser storage.
///
/// ## Versioning
///
/// Versioning is the process of handling long-term data that can outlive our code.
///
/// For example we could have a settings struct whose members change over time. We might eventually add timezone support and we might then remove support for a thousand separator on numbers. Each change results in a new possible version of the stored data. If we stored these settings in browser storage we would need to handle all possible versions of the data format that can occur. If we don't offer versioning then all settings could revert to the default every time we encounter an old format.
///
/// How best to handle versioning depends on the codec involved:
///
/// - The [`StringCodec`](super::StringCodec) can avoid versioning entirely by keeping to privimitive types. In our example above, we could have decomposed the settings struct into separate timezone and number separator fields. These would be encoded as strings and stored as two separate key-value fields in the browser rather than a single field. If a field is missing then the value intentionally would fallback to the default without interfering with the other field.
///
/// - The [`ProstCodec`](super::ProstCodec) uses [Protocol buffers](https://protobuf.dev/overview/) designed to solve the problem of long-term storage. It provides semantics for versioning that are not present in JSON or other formats.
///
/// - The [`JsonCodec`](super::JsonCodec) stores data as JSON. We can then rely on serde or by providing our own manual version handling. See the codec for more details.
pub trait Codec<T>: Clone + 'static {
    /// The error type returned when encoding or decoding fails.
    type Error;
    /// Encodes a value to a UTF-16 string.
    fn encode(&self, val: &T) -> Result<String, Self::Error>;
    /// Decodes a UTF-16 string to a value. Should be able to decode any string encoded by [`encode`].
    fn decode(&self, str: String) -> Result<T, Self::Error>;
}

/// Calls the on_error callback with the given error. Removes the error from the Result to avoid double error handling.
#[cfg(not(feature = "ssr"))]
fn handle_error<T, Err>(
    on_error: &Rc<dyn Fn(UseStorageError<Err>)>,
    result: Result<T, UseStorageError<Err>>,
) -> Result<T, ()> {
    result.map_err(|err| (on_error)(err))
}

impl<T: Default, C: Codec<T> + Default> Default for UseStorageOptions<T, C> {
    fn default() -> Self {
        Self {
            codec: C::default(),
            on_error: Rc::new(|_err| ()),
            listen_to_storage_changes: true,
            initial_value: MaybeRwSignal::default(),
            filter: FilterOptions::default(),
        }
    }
}

impl<T: Default, C: Codec<T>> UseStorageOptions<T, C> {
    /// Sets the codec to use for encoding and decoding values to and from UTF-16 strings.
    pub fn codec(self, codec: impl Into<C>) -> Self {
        Self {
            codec: codec.into(),
            ..self
        }
    }

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

    /// Initial value to use when the storage key is not set. Note that this value is read once on creation of the storage hook and not updated again. Accepts a signal and defaults to `T::default()`.
    pub fn initial_value(self, initial: impl Into<MaybeRwSignal<T>>) -> Self {
        Self {
            initial_value: initial.into(),
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
