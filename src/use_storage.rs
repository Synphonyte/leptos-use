use crate::{use_event_listener_with_options, use_window, UseEventListenerOptions};
use leptos::*;
use std::rc::Rc;
use thiserror::Error;
use wasm_bindgen::JsValue;
use web_sys::Storage;

#[derive(Clone)]
pub struct UseStorageOptions<Err> {
    on_error: Rc<dyn Fn(UseStorageError<Err>)>,
    listen_to_storage_changes: bool,
}

/// Session handling errors returned by [`use_storage`].
#[derive(Error, Debug)]
pub enum UseStorageError<Err> {
    #[error("window not available")]
    WindowReturnedNone,
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
    #[error("failed to parse item value")]
    ParseItemError(Err),
}

/// Hook for using local storage. Returns a result of a signal and a setter / deleter.
pub fn use_local_storage<T>(key: impl AsRef<str>) -> (Memo<T>, impl Fn(Option<T>) -> ())
where
    T: Clone + Default + PartialEq + TryFrom<String> + ToString,
    T::Error: std::fmt::Debug,
{
    use_local_storage_with_options(key, UseStorageOptions::default())
}

/// Hook for using local storage. Returns a result of a signal and a setter / deleter.
pub fn use_local_storage_with_options<T>(
    key: impl AsRef<str>,
    options: UseStorageOptions<T::Error>,
) -> (Memo<T>, impl Fn(Option<T>) -> ())
where
    T: Clone + Default + PartialEq + TryFrom<String> + ToString,
{
    // TODO ssr
    let UseStorageOptions {
        on_error,
        listen_to_storage_changes,
    } = options;
    let storage: Result<Storage, ()> = handle_error(&on_error, try_storage());

    let initial_value = storage
        .to_owned()
        // Get initial item from storage
        .and_then(|s| {
            let result = s
                .get_item(key.as_ref())
                .map_err(UseStorageError::GetItemFailed);
            handle_error(&on_error, result)
        })
        .unwrap_or_default();
    // Attempt to parse the item string
    let initial_value = parse_item(initial_value, &on_error);
    let (data, set_data) = create_signal(initial_value);

    // Update storage value
    let set_value = {
        let storage = storage.to_owned();
        let key = key.as_ref().to_owned();
        let on_error = on_error.to_owned();
        move |value: Option<T>| {
            let key = key.as_str();
            // Attempt to update storage
            let _ = storage.as_ref().map(|storage| {
                let result = match value {
                    // Update
                    Some(ref value) => storage
                        .set_item(key, &value.to_string())
                        .map_err(UseStorageError::SetItemFailed),
                    // Remove
                    None => storage
                        .remove_item(key)
                        .map_err(UseStorageError::RemoveItemFailed),
                };
                handle_error(&on_error, result)
            });

            // Notify signal of change
            set_data.set(value);
        }
    };

    // Listen for storage events
    // Note: we only receive events from other tabs / windows, not from internal updates.
    if listen_to_storage_changes {
        let key = key.as_ref().to_owned();
        let _ = use_event_listener_with_options(
            use_window(),
            leptos::ev::storage,
            move |ev| {
                // Update storage value if our key matches
                if let Some(k) = ev.key() {
                    if k == key {
                        let value = parse_item(ev.new_value(), &on_error);
                        set_data.set(value)
                    }
                } else {
                    // All keys deleted
                    set_data.set(None)
                }
            },
            UseEventListenerOptions::default().passive(true),
        );
    };

    let value = create_memo(move |_| data.get().unwrap_or_default());
    (value, set_value)
}

fn try_storage<Err>() -> Result<Storage, UseStorageError<Err>> {
    use_window()
        .as_ref()
        .ok_or_else(|| UseStorageError::WindowReturnedNone)?
        .local_storage()
        .map_err(|err| UseStorageError::StorageNotAvailable(err))?
        .ok_or_else(|| UseStorageError::StorageReturnedNone)
}

/// Calls the on_error callback with the given error. Removes the error from the Result to avoid double error handling.
fn handle_error<T, Err>(
    on_error: &Rc<dyn Fn(UseStorageError<Err>)>,
    result: Result<T, UseStorageError<Err>>,
) -> Result<T, ()> {
    result.or_else(|err| Err((on_error)(err)))
}

fn parse_item<T: Default + TryFrom<String>>(
    str: Option<String>,
    on_error: &Rc<dyn Fn(UseStorageError<T::Error>)>,
) -> Option<T> {
    str.map(|str| {
        let result = T::try_from(str).map_err(UseStorageError::ParseItemError);
        handle_error(&on_error, result)
    })
    .transpose()
    // We've sent our error so unwrap to drop () error
    .unwrap_or_default()
}

impl<Err: std::fmt::Debug> Default for UseStorageOptions<Err> {
    fn default() -> Self {
        Self {
            on_error: Rc::new(|_err| ()),
            listen_to_storage_changes: true,
        }
    }
}

impl<Err> UseStorageOptions<Err> {
    pub fn on_error(self, on_error: impl Fn(UseStorageError<Err>) + 'static) -> Self {
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
}
