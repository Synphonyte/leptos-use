use crate::{
    core::{MaybeRwSignal, StorageType},
    use_event_listener, use_window,
};
use cfg_if::cfg_if;
use leptos::*;
use std::rc::Rc;
use thiserror::Error;
use wasm_bindgen::JsValue;

const INTERNAL_STORAGE_EVENT: &str = "leptos-use-storage";

pub trait Codec<T>: Clone + 'static {
    type Error;
    fn encode(&self, val: &T) -> Result<String, Self::Error>;
    fn decode(&self, str: String) -> Result<T, Self::Error>;
}

#[derive(Clone)]
pub struct UseStorageOptions<T: 'static, C: Codec<T>> {
    codec: C,
    on_error: Rc<dyn Fn(UseStorageError<C::Error>)>,
    listen_to_storage_changes: bool,
    default_value: MaybeRwSignal<T>,
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

/// Hook for using local storage. Returns a result of a signal and a setter / deleter.
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

/// Hook for using session storage. Returns a result of a signal and a setter / deleter.
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

/// Hook for using any kind of storage. Returns a result of a signal and a setter / deleter.
pub fn use_storage_with_options<T, C>(
    storage_type: StorageType,
    key: impl AsRef<str>,
    options: UseStorageOptions<T, C>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() -> () + Clone)
where
    T: Clone + PartialEq,
    C: Codec<T>,
{
    cfg_if! { if #[cfg(feature = "ssr")] {
        let (data, set_data) = create_signal(None);
        let set_value = move |value: Option<T>| {
            set_data.set(value);
        };
        let value = create_memo(move |_| data.get().unwrap_or_default());
        return (value, set_value, || ());
    } else {
        // Continue
    }}

    let UseStorageOptions {
        codec,
        on_error,
        listen_to_storage_changes,
        default_value,
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
        let _ = watch(
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
            false,
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
        Self::new(C::default())
    }
}

impl<T: Default, C: Codec<T>> UseStorageOptions<T, C> {
    pub(super) fn new(codec: C) -> Self {
        Self {
            codec,
            on_error: Rc::new(|_err| ()),
            listen_to_storage_changes: true,
            default_value: MaybeRwSignal::default(),
        }
    }

    pub fn on_error(self, on_error: impl Fn(UseStorageError<C::Error>) + 'static) -> Self {
        Self {
            on_error: Rc::new(on_error),
            ..self
        }
    }

    pub fn listen_to_storage_changes(self, listen_to_storage_changes: bool) -> Self {
        Self {
            listen_to_storage_changes,
            ..self
        }
    }

    pub fn default_value(self, values: impl Into<MaybeRwSignal<T>>) -> Self {
        Self {
            default_value: values.into(),
            ..self
        }
    }
}
