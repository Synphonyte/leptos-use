#[cfg(not(feature = "ssr"))]
use crate::UseStorageError;
use crate::{CallFilter, StorageType, UseStorageOptions};
#[cfg(not(feature = "ssr"))]
use codee::CodecError;
use codee::{Decoder, Encoder};
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
#[cfg(not(feature = "ssr"))]
use std::sync::Arc;
#[cfg(not(feature = "ssr"))]
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
/// is then encoded and decoded via the given `*Codec`. This value is synced with other calls using
/// the same key on the same page and across tabs for local storage.
/// See [`UseStorageOptions`] to see how behavior can be further customised.
///
/// Values are (en)decoded via the given codec. You can use any of the string codecs or a
/// binary codec wrapped in `Base64`.
///
/// > Please check [the codec chapter](https://leptos-use.rs/codecs.html) to see what codecs are
/// > available and what feature flags they require.
///
/// ## Example
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_storage::{StorageType, use_local_storage, use_session_storage, use_storage};
/// # use serde::{Deserialize, Serialize};
/// # use codee::string::{FromToStringCodec, JsonSerdeCodec, Base64};
/// # use codee::binary::ProstCodec;
/// #
/// # #[component]
/// # pub fn Demo() -> impl IntoView {
/// // Binds a struct:
/// let (state, set_state, _) = use_local_storage::<MyState, JsonSerdeCodec>("my-state");
///
/// // Binds a bool, stored as a string:
/// let (flag, set_flag, remove_flag) = use_session_storage::<bool, FromToStringCodec>("my-flag");
///
/// // Binds a number, stored as a string:
/// let (count, set_count, _) = use_session_storage::<i32, FromToStringCodec>("my-count");
/// // Binds a number, stored in JSON:
/// let (count, set_count, _) = use_session_storage::<i32, JsonSerdeCodec>("my-count-kept-in-js");
///
/// // Bind string with SessionStorage stored in ProtoBuf format:
/// let (id, set_id, _) = use_storage::<String, Base64<ProstCodec>>(
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
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server the returned signals will just read/manipulate the `initial_value` without persistence.
///
/// ### Hydration bugs and `use_cookie`
///
/// If you use a value from storage to control conditional rendering you might run into issues with
/// hydration.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_storage::use_session_storage;
/// # use codee::string::FromToStringCodec;
/// #
/// # #[component]
/// # pub fn Example() -> impl IntoView {
/// let (flag, set_flag, _) = use_session_storage::<bool, FromToStringCodec>("my-flag");
///
/// view! {
///     <Show when=move || flag.get()>
///         <div>Some conditional content</div>
///     </Show>
/// }
/// # }
/// ```
///
/// You can see hydration warnings in the browser console and the conditional parts of
/// the app might never show up when rendered on the server and then hydrated in the browser. The
/// reason for this is that the server has no access to storage and therefore will always use
/// `initial_value` as described above. So on the server your app is always rendered as if
/// the value from storage was `initial_value`. Then in the browser the actual stored value is used
/// which might be different, hence during hydration the DOM looks different from the one rendered
/// on the server which produces the hydration warnings.
///
/// The recommended way to avoid this is to use `use_cookie` instead because values stored in cookies
/// are available on the server as well as in the browser.
///
/// If you still want to use storage instead of cookies you can use the `delay_during_hydration`
/// option that will use the `initial_value` during hydration just as on the server and delay loading
/// the value from storage by an animation frame. This gets rid of the hydration warnings and makes
/// the app correctly render things. Some flickering might be unavoidable though.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_storage::{use_local_storage_with_options, UseStorageOptions};
/// # use codee::string::FromToStringCodec;
/// #
/// # #[component]
/// # pub fn Example() -> impl IntoView {
/// let (flag, set_flag, _) = use_local_storage_with_options::<bool, FromToStringCodec>(
///     "my-flag",
///     UseStorageOptions::default().delay_during_hydration(true),
/// );
///
/// view! {
///     <Show when=move || flag.get()>
///         <div>Some conditional content</div>
///     </Show>
/// }
/// # }
/// ```
#[inline(always)]
pub fn use_storage<T, C>(
    storage_type: StorageType,
    key: impl Into<Signal<String>>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() + Clone + Send + Sync)
where
    T: Default + Clone + PartialEq + Send + Sync + 'static,
    C: Encoder<T, Encoded = String> + Decoder<T, Encoded = str>,
{
    use_storage_with_options::<T, C>(storage_type, key, UseStorageOptions::default())
}

/// Version of [`use_storage`] that accepts [`UseStorageOptions`].
pub fn use_storage_with_options<T, C>(
    storage_type: StorageType,
    key: impl Into<Signal<String>>,
    options: UseStorageOptions<
        T,
        <C as Encoder<T>>::Error,
        <C as Decoder<T>>::Error,
        impl CallFilter,
    >,
) -> (Signal<T>, WriteSignal<T>, impl Fn() + Clone + Send + Sync)
where
    T: Clone + PartialEq + Send + Sync,
    C: Encoder<T, Encoded = String> + Decoder<T, Encoded = str>,
{
    let UseStorageOptions {
        on_error,
        listen_to_storage_changes,
        initial_value,
        filter,
        delay_during_hydration,
    } = options;

    let (data, set_data) = initial_value.into_signal();
    let default = data.get_untracked();

    #[cfg(feature = "ssr")]
    {
        let _ = on_error;
        let _ = listen_to_storage_changes;
        let _ = filter;
        let _ = delay_during_hydration;
        let _ = storage_type;
        let _ = key;
        let _ = INTERNAL_STORAGE_EVENT;

        let remove = move || {
            set_data.set(default.clone());
        };

        (data, set_data, remove)
    }

    #[cfg(not(feature = "ssr"))]
    {
        use crate::{
            UseEventListenerOptions, WatchOptions, sendwrap_fn, use_event_listener_with_options,
            use_window, watch_with_options,
        };
        use send_wrapper::SendWrapper;

        let delaying = StoredValue::new(
            delay_during_hydration
                && Owner::current_shared_context()
                    .map(|sc| sc.during_hydration())
                    .unwrap_or_default(),
        );

        let key = key.into();

        // Get storage API
        let storage = storage_type
            .into_storage()
            .map_err(UseStorageError::StorageNotAvailable)
            .and_then(|s| s.ok_or(UseStorageError::StorageReturnedNone));
        let storage = handle_error(&on_error, storage);

        // Schedules a storage event microtask. Uses a queue to avoid re-entering the runtime
        let dispatch_storage_event = {
            let on_error = on_error.to_owned();

            move || {
                let on_error = on_error.to_owned();

                queue_microtask(move || {
                    // TODO : better to use a BroadcastChannel (use_broadcast_channel)?
                    // Note: we cannot construct a full StorageEvent so we _must_ rely on a custom event
                    let custom = web_sys::CustomEventInit::new();
                    custom.set_detail(&JsValue::from_str(&key.get_untracked()));
                    let result = web_sys::CustomEvent::new_with_event_init_dict(
                        INTERNAL_STORAGE_EVENT,
                        &custom,
                    )
                    .and_then(|event| window().dispatch_event(&event))
                    .map_err(UseStorageError::NotifyItemChangedFailed);
                    let _ = handle_error(&on_error, result);
                })
            }
        };

        let read_from_storage = {
            let storage = storage.to_owned();
            let on_error = on_error.to_owned();

            move || {
                storage
                    .to_owned()
                    .and_then(|storage| {
                        // Get directly from storage
                        let result = storage
                            .get_item(&key.get_untracked())
                            .map_err(UseStorageError::GetItemFailed);
                        handle_error(&on_error, result)
                    })
                    .unwrap_or_default() // Drop handled Err(())
                    .as_ref()
                    .map(|encoded| {
                        // Decode item
                        let result = C::decode(encoded)
                            .map_err(|e| UseStorageError::ItemCodecError(CodecError::Decode(e)));
                        handle_error(&on_error, result)
                    })
                    .transpose()
                    .unwrap_or_default() // Drop handled Err(())
            }
        };

        // Fetches direct from browser storage and fills set_data if changed (memo)
        let fetch_from_storage = {
            let default = default.clone();
            let read_from_storage = read_from_storage.clone();

            SendWrapper::new(move || {
                let fetched = read_from_storage();

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
            })
        };

        // Fires when storage needs to be fetched
        let notify = ArcTrigger::new();

        // Refetch from storage. Keeps track of how many times we've been notified. Does not increment for calls to set_data
        let notify_id = Memo::<usize>::new({
            let notify = notify.clone();
            let fetch_from_storage = fetch_from_storage.clone();

            move |prev| {
                notify.track();
                match prev {
                    None => 1, // Avoid async fetch of initial value
                    Some(prev) => {
                        fetch_from_storage();
                        prev + 1
                    }
                }
            }
        });

        // Notify id as of hook setup. The watch below runs its first tick deferred, so a
        // notification can race in before then. Comparing against this baseline lets that
        // first tick recognize an external change instead of assuming there was none.
        let initial_notify_id = notify_id.get_untracked();

        // Set item on internal (non-event) page changes to the data signal
        {
            let storage = storage.to_owned();
            let on_error = on_error.to_owned();
            let dispatch_storage_event = dispatch_storage_event.to_owned();
            let default = default.clone();

            let _ = watch_with_options(
                move || (notify_id.get(), data.get()),
                move |(id, value), prev, _| {
                    // Skip setting storage on changes from external events. The ID will change on external events.
                    let change_from_external_event = prev
                        .map(|(prev_id, _)| *prev_id != *id)
                        .unwrap_or_else(|| *id != initial_notify_id);

                    if change_from_external_event {
                        return;
                    }

                    // Don't write default value if store is empty or still hydrating
                    if value == &default && (read_from_storage().is_none() || delaying.get_value())
                    {
                        return;
                    }

                    if let Ok(storage) = &storage {
                        // Encode value
                        let result = C::encode(value)
                            .map_err(|e| UseStorageError::ItemCodecError(CodecError::Encode(e)))
                            .and_then(|enc_value| {
                                // Set storage -- sends a global event
                                storage
                                    .set_item(&key.get_untracked(), &enc_value)
                                    .map_err(UseStorageError::SetItemFailed)
                            });
                        let result = handle_error(&on_error, result);
                        // Send internal storage event
                        if result.is_ok() {
                            dispatch_storage_event();
                        }
                    }
                },
                WatchOptions::default().filter(filter).immediate(true),
            );
        }

        if delaying.get_value() {
            let _ = request_animation_frame({
                let fetch_from_storage = fetch_from_storage.clone();
                move || {
                    delaying.set_value(false);
                    fetch_from_storage()
                }
            });
        } else {
            fetch_from_storage();
        }

        if listen_to_storage_changes {
            let listener_options = UseEventListenerOptions::default().passive(true);
            // Listen to global storage events
            let _ = use_event_listener_with_options(
                use_window(),
                leptos::ev::storage,
                {
                    let notify = notify.clone();

                    move |ev| {
                        let ev_key = ev.key();
                        // Key matches or all keys deleted (None)
                        if ev_key == Some(key.get_untracked()) || ev_key.is_none() {
                            notify.notify()
                        }
                    }
                },
                listener_options,
            );
            // Listen to internal storage events
            let _ = use_event_listener_with_options(
                use_window(),
                leptos::ev::Custom::new(INTERNAL_STORAGE_EVENT),
                {
                    let notify = notify.clone();

                    move |ev: web_sys::CustomEvent| {
                        if Some(key.get_untracked()) == ev.detail().as_string() {
                            notify.notify()
                        }
                    }
                },
                listener_options,
            );
        };

        Effect::watch(
            move || key.get(),
            {
                let notify = notify.clone();
                move |_, _, _| notify.notify()
            },
            false,
        );

        // Remove from storage fn
        let remove = {
            sendwrap_fn!(move || {
                // Reset the signal right away just like on the server. The
                // notification below only refetches on the next reactive tick.
                set_data.set(default.clone());

                let _ = storage.as_ref().map(|storage| {
                    // Delete directly from storage
                    let result = storage
                        .remove_item(&key.get_untracked())
                        .map_err(UseStorageError::RemoveItemFailed);
                    let _ = handle_error(&on_error, result);
                    notify.notify();
                    dispatch_storage_event();
                });
            })
        };

        (data, set_data, remove)
    }
}

/// Calls the on_error callback with the given error. Removes the error from the Result to avoid double error handling.
#[cfg(not(feature = "ssr"))]
fn handle_error<T, E, D>(
    on_error: &Arc<dyn Fn(UseStorageError<E, D>) + Send + Sync>,
    result: Result<T, UseStorageError<E, D>>,
) -> Result<T, ()> {
    result.map_err(|err| (on_error)(err))
}
