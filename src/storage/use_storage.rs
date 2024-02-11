use crate::{
    core::{MaybeRwSignal, StorageType},
    utils::{FilterOptions, StringCodec},
};
use cfg_if::cfg_if;
use leptos::*;
use std::rc::Rc;
use thiserror::Error;
use wasm_bindgen::JsValue;

const INTERNAL_STORAGE_EVENT: &str = "leptos-use-storage";

/// Reactive [Storage](https://developer.mozilla.org/en-US/docs/Web/API/Storage).
///
/// The function returns a triplet `(read_signal, write_signal, delete_from_storage_fn)`.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_storage)
///
/// ## Usage
///
/// Pass a [`StorageType`] to determine the kind of key-value browser storage to use.
/// The specified key is where data is stored. All values are stored as UTF-16 strings which
/// is then encoded and decoded via the given [`Codec`]. This value is synced with other calls using
/// the same key on the smae page and across tabs for local storage.
/// See [`UseStorageOptions`] to see how behaviour can be further customised.
///
/// See [`StringCodec`] for more details on how to handle versioning — dealing with data that can outlast your code.
///
/// > To use the [`JsonCodec`], you will need to add the `"serde"` feature to your project's `Cargo.toml`.
/// > To use [`ProstCodec`], add the feature `"prost"`.
///
/// ## Example
///
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage};
/// # use serde::{Deserialize, Serialize};
/// # use leptos_use::utils::{FromToStringCodec, JsonCodec, ProstCodec};
/// #
/// # pub fn Demo() -> impl IntoView {
/// // Binds a struct:
/// let (state, set_state, _) = use_local_storage::<MyState, JsonCodec>("my-state");
///
/// // Binds a bool, stored as a string:
/// let (flag, set_flag, remove_flag) = use_session_storage::<bool, FromToStringCodec>("my-flag");
///
/// // Binds a number, stored as a string:
/// let (count, set_count, _) = use_session_storage::<i32, FromToStringCodec>("my-count");
/// // Binds a number, stored in JSON:
/// let (count, set_count, _) = use_session_storage::<i32, JsonCodec>("my-count-kept-in-js");
///
/// // Bind string with SessionStorage stored in ProtoBuf format:
/// let (id, set_id, _) = use_storage::<String, ProstCodec>(
///     StorageType::Session,
///     "my-id",
/// );
/// #    view! { }
/// # }
///
/// // Data stored in JSON must implement Serialize, Deserialize.
/// // And you have to add the feature "serde" to your project's Cargo.toml
/// #[derive(Serialize, Deserialize, Clone, PartialEq)]
/// pub struct MyState {
///     pub hello: String,
///     pub greeting: String,
/// }
///
/// // Default can be used to implement initial or deleted values.
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
///
/// ## Create Your Own Custom Codec
///
/// All you need to do is to implement the [`StringCodec`] trait together with `Default` and `Clone`.
///
/// ## Server-Side Rendering
///
/// On the server the returned signals will just read/manipulate the `initial_value` without persistence.
#[inline(always)]
pub fn use_storage<T, C>(
    storage_type: StorageType,
    key: impl AsRef<str>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() + Clone)
where
    T: Default + Clone + PartialEq,
    C: StringCodec<T> + Default,
{
    use_storage_with_options::<T, C>(storage_type, key, UseStorageOptions::default())
}

/// Version of [`use_storage`] that accepts [`UseStorageOptions`].
pub fn use_storage_with_options<T, C>(
    storage_type: StorageType,
    key: impl AsRef<str>,
    options: UseStorageOptions<T, C>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() + Clone)
where
    T: Clone + PartialEq,
    C: StringCodec<T> + Default,
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
pub struct UseStorageOptions<T: 'static, C: StringCodec<T>> {
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

/// Calls the on_error callback with the given error. Removes the error from the Result to avoid double error handling.
#[cfg(not(feature = "ssr"))]
fn handle_error<T, Err>(
    on_error: &Rc<dyn Fn(UseStorageError<Err>)>,
    result: Result<T, UseStorageError<Err>>,
) -> Result<T, ()> {
    result.map_err(|err| (on_error)(err))
}

impl<T: Default, C: StringCodec<T> + Default> Default for UseStorageOptions<T, C> {
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

impl<T: Default, C: StringCodec<T>> UseStorageOptions<T, C> {
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
