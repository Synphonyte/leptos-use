use crate::utils::{CloneableFn, CloneableFnWithArg, FilterOptions};
use crate::{
    filter_builder_methods, use_event_listener, watch_pausable_with_options, DebounceOptions,
    ThrottleOptions, WatchOptions, WatchPausableReturn,
};
use default_struct_builder::DefaultBuilder;
use js_sys::Reflect;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::time::Duration;
use wasm_bindgen::{JsCast, JsValue};

pub use crate::core::StorageType;

const CUSTOM_STORAGE_EVENT_NAME: &str = "leptos-use-storage";

/// Reactive [LocalStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage) / [SessionStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_storage)
///
/// ## Usage
///
/// It returns a triplet `(read_signal, write_signal, delete_from_storage_func)` of type `(ReadSignal<T>, WriteSignal<T>, Fn())`.
///
/// Values are (de-)serialized to/from JSON using [`serde`](https://serde.rs/).
///
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{StorageType, use_storage, use_storage_with_options, UseStorageOptions};
/// # use serde::{Deserialize, Serialize};
/// #
/// #[derive(Serialize, Deserialize, Clone)]
/// pub struct MyState {
///     pub hello: String,
///     pub greeting: String,
/// }
///
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// // bind struct. Must be serializable.
/// let (state, set_state, _) = use_storage(
///     cx,
///     "my-state",
///     MyState {
///         hello: "hi".to_string(),
///         greeting: "Hello".to_string()
///     },
/// ); // returns Signal<MyState>
///
/// // bind bool.
/// let (flag, set_flag, remove_flag) = use_storage(cx, "my-flag", true); // returns Signal<bool>
///
/// // bind number
/// let (count, set_count, _) = use_storage(cx, "my-count", 0); // returns Signal<i32>
///
/// // bind string with SessionStorage
/// let (id, set_id, _) = use_storage_with_options(
///     cx,
///     "my-id",
///     "some_string_id".to_string(),
///     UseStorageOptions::default().storage_type(StorageType::Session),
/// );
/// #    view! { cx, }
/// # }
/// ```
///
/// ## Merge Defaults
///
/// By default, [`use_storage`] will use the value from storage if it is present and ignores the default value.
/// Be aware that when you add more properties to the default value, the key might be `None`
/// (in the case of an `Option<T>` field) if client's storage does not have that key
/// or deserialization might fail altogether.
///
/// Let's say you had a struct `MyState` that has been saved to storage
///
/// ```ignore
/// #[derive(Serialize, Deserialize, Clone)]
/// struct MyState {
///     hello: String,
/// }
///
/// let (state, .. ) = use_storage(cx, "my-state", MyState { hello: "hello" });
/// ```
///
/// Now, in a newer version you added a field `greeting` to `MyState`.
///
/// ```ignore
/// #[derive(Serialize, Deserialize, Clone)]
/// struct MyState {
///     hello: String,
///     greeting: String,
/// }
///
/// let (state, .. ) = use_storage(
///     cx,
///     "my-state",
///     MyState { hello: "hi", greeting: "whatsup" },
/// ); // fails to deserialize -> default value
/// ```
///
/// This will fail to deserialize the stored string `{"hello": "hello"}` because it has no field `greeting`.
/// Hence it just uses the new default value provided and the previously saved value is lost.
///
/// To mitigate that you can provide a `merge_defaults` option. This is a pure function pointer
/// that takes the serialized (to json) stored value and the default value as arguments
/// and should return the serialized merged value.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{use_storage_with_options, UseStorageOptions};
/// # use serde::{Deserialize, Serialize};
/// #
/// #[derive(Serialize, Deserialize, Clone)]
/// pub struct MyState {
///     pub hello: String,
///     pub greeting: String,
/// }
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// let (state, set_state, _) = use_storage_with_options(
///     cx,
///     "my-state",
///     MyState {
///         hello: "hi".to_string(),
///         greeting: "Hello".to_string()
///     },
///     UseStorageOptions::<MyState>::default().merge_defaults(|stored_value, default_value| {
///         if stored_value.contains(r#""greeting":"#) {
///             stored_value.to_string()
///         } else {
///             // add "greeting": "Hello" to the string
///             stored_value.replace("}", &format!(r#""greeting": "{}"}}"#, default_value.greeting))
///         }
///     }),
/// );
/// #
/// #    view! { cx, }
/// # }
/// ```
///
/// ## Filter Storage Write
///
/// You can specify `debounce` or `throttle` options for limiting writes to storage.
///
/// ## See also
///
/// * [`use_local_storage`]
/// * [`use_session_storage`]
// #[doc(cfg(feature = "storage"))]
pub fn use_storage<T, D>(
    cx: Scope,
    key: &str,
    defaults: D,
) -> (ReadSignal<T>, WriteSignal<T>, impl Fn() + Clone)
where
    for<'de> T: Serialize + Deserialize<'de> + Clone + 'static,
    D: Into<MaybeSignal<T>>,
    T: Clone,
{
    use_storage_with_options(cx, key, defaults, UseStorageOptions::default())
}

/// Version of [`use_storage`] that accepts [`UseStorageOptions`]. See [`use_storage`] for how to use.
// #[doc(cfg(feature = "storage"))]
pub fn use_storage_with_options<T, D>(
    cx: Scope,
    key: &str,
    defaults: D,
    options: UseStorageOptions<T>,
) -> (ReadSignal<T>, WriteSignal<T>, impl Fn() + Clone)
where
    for<'de> T: Serialize + Deserialize<'de> + Clone + 'static,
    D: Into<MaybeSignal<T>>,
    T: Clone,
{
    let defaults = defaults.into();

    let UseStorageOptions {
        storage_type,
        listen_to_storage_changes,
        write_defaults,
        merge_defaults,
        on_error,
        filter,
    } = options;

    let (data, set_data) = create_signal(cx, defaults.get_untracked());

    let storage = storage_type.into_storage();

    let remove: Box<dyn CloneableFn> = match storage {
        Ok(Some(storage)) => {
            let on_err = on_error.clone();

            let store = storage.clone();
            let k = key.to_string();

            let write = move |v: &T| {
                match serde_json::to_string(&v) {
                    Ok(ref serialized) => match store.get_item(&k) {
                        Ok(old_value) => {
                            if old_value.as_ref() != Some(serialized) {
                                if let Err(e) = store.set_item(&k, serialized) {
                                    on_err(UseStorageError::StorageAccessError(e));
                                } else {
                                    let mut event_init = web_sys::CustomEventInit::new();
                                    event_init.detail(
                                        &StorageEventDetail {
                                            key: Some(k.clone()),
                                            old_value,
                                            new_value: Some(serialized.clone()),
                                            storage_area: Some(store.clone()),
                                        }
                                        .into(),
                                    );

                                    // importantly this should _not_ be a StorageEvent since those cannot
                                    // be constructed with a non-built-in storage area
                                    let _ = window().dispatch_event(
                                        &web_sys::CustomEvent::new_with_event_init_dict(
                                            CUSTOM_STORAGE_EVENT_NAME,
                                            &event_init,
                                        )
                                        .expect("Failed to create CustomEvent"),
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            on_err.clone()(UseStorageError::StorageAccessError(e));
                        }
                    },
                    Err(e) => {
                        on_err.clone()(UseStorageError::SerializationError(e));
                    }
                }
            };

            let store = storage.clone();
            let on_err = on_error.clone();
            let k = key.to_string();
            let def = defaults.clone();

            let read = move |event_detail: Option<StorageEventDetail>| -> Option<T> {
                let raw_init = match serde_json::to_string(&def.get_untracked()) {
                    Ok(serialized) => Some(serialized),
                    Err(e) => {
                        on_err.clone()(UseStorageError::DefaultSerializationError(e));
                        None
                    }
                };

                let raw_value = if let Some(event_detail) = event_detail {
                    event_detail.new_value
                } else {
                    match store.get_item(&k) {
                        Ok(raw_value) => match raw_value {
                            Some(raw_value) => {
                                Some(merge_defaults(&raw_value, &def.get_untracked()))
                            }
                            None => raw_init.clone(),
                        },
                        Err(e) => {
                            on_err.clone()(UseStorageError::StorageAccessError(e));
                            None
                        }
                    }
                };

                match raw_value {
                    Some(raw_value) => match serde_json::from_str(&raw_value) {
                        Ok(v) => Some(v),
                        Err(e) => {
                            on_err.clone()(UseStorageError::SerializationError(e));
                            None
                        }
                    },
                    None => {
                        if let Some(raw_init) = &raw_init {
                            if write_defaults {
                                if let Err(e) = store.set_item(&k, raw_init) {
                                    on_err(UseStorageError::StorageAccessError(e));
                                }
                            }
                        }

                        Some(def.get_untracked())
                    }
                }
            };

            let WatchPausableReturn {
                pause: pause_watch,
                resume: resume_watch,
                ..
            } = watch_pausable_with_options(
                cx,
                move || data.get(),
                move |data, _, _| write.clone()(data),
                WatchOptions::default().filter(filter),
            );

            let k = key.to_string();
            let store = storage.clone();

            let update = move |event_detail: Option<StorageEventDetail>| {
                if let Some(event_detail) = &event_detail {
                    if event_detail.storage_area != Some(store) {
                        return;
                    }

                    match &event_detail.key {
                        None => {
                            set_data.set(defaults.get_untracked());
                            return;
                        }
                        Some(event_key) => {
                            if event_key != &k {
                                return;
                            }
                        }
                    };
                }

                pause_watch();

                if let Some(value) = read(event_detail.clone()) {
                    set_data.set(value);
                }

                if event_detail.is_some() {
                    // use timeout to avoid inifinite loop
                    let resume = resume_watch.clone();
                    let _ = set_timeout_with_handle(resume, Duration::ZERO);
                } else {
                    resume_watch();
                }
            };

            let upd = update.clone();
            let update_from_custom_event =
                move |event: web_sys::CustomEvent| upd.clone()(Some(event.into()));

            let upd = update.clone();
            let update_from_storage_event =
                move |event: web_sys::StorageEvent| upd.clone()(Some(event.into()));

            if listen_to_storage_changes {
                let _ = use_event_listener(cx, window(), ev::storage, update_from_storage_event);
                let _ = use_event_listener(
                    cx,
                    window(),
                    ev::Custom::new(CUSTOM_STORAGE_EVENT_NAME),
                    update_from_custom_event,
                );
            }

            update(None);

            let k = key.to_string();

            Box::new(move || {
                let _ = storage.remove_item(&k);
            })
        }
        Err(e) => {
            on_error(UseStorageError::NoStorage(e));
            Box::new(move || {})
        }
        _ => {
            // do nothing
            Box::new(move || {})
        }
    };

    (data, set_data, move || remove.clone()())
}

#[derive(Clone)]
pub struct StorageEventDetail {
    pub key: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub storage_area: Option<web_sys::Storage>,
}

impl From<web_sys::StorageEvent> for StorageEventDetail {
    fn from(event: web_sys::StorageEvent) -> Self {
        Self {
            key: event.key(),
            old_value: event.old_value(),
            new_value: event.new_value(),
            storage_area: event.storage_area(),
        }
    }
}

impl From<web_sys::CustomEvent> for StorageEventDetail {
    fn from(event: web_sys::CustomEvent) -> Self {
        let detail = event.detail();
        Self {
            key: get_optional_string(&detail, "key"),
            old_value: get_optional_string(&detail, "oldValue"),
            new_value: get_optional_string(&detail, "newValue"),
            storage_area: Reflect::get(&detail, &"storageArea".into())
                .map(|v| v.dyn_into::<web_sys::Storage>().ok())
                .unwrap_or_default(),
        }
    }
}

impl From<StorageEventDetail> for JsValue {
    fn from(event: StorageEventDetail) -> Self {
        let obj = js_sys::Object::new();

        let _ = Reflect::set(&obj, &"key".into(), &event.key.into());
        let _ = Reflect::set(&obj, &"oldValue".into(), &event.old_value.into());
        let _ = Reflect::set(&obj, &"newValue".into(), &event.new_value.into());
        let _ = Reflect::set(&obj, &"storageArea".into(), &event.storage_area.into());

        obj.into()
    }
}

fn get_optional_string(v: &JsValue, key: &str) -> Option<String> {
    Reflect::get(v, &key.into())
        .map(|v| v.as_string())
        .unwrap_or_default()
}

/// Error type for use_storage_with_options
// #[doc(cfg(feature = "storage"))]
pub enum UseStorageError<E = ()> {
    NoStorage(JsValue),
    StorageAccessError(JsValue),
    CustomStorageAccessError(E),
    SerializationError(Error),
    DefaultSerializationError(Error),
}

/// Options for [`use_storage_with_options`].
// #[doc(cfg(feature = "storage"))]
#[derive(DefaultBuilder)]
pub struct UseStorageOptions<T> {
    /// Type of storage. Can be `Local` (default), `Session` or `Custom(web_sys::Storage)`
    pub(crate) storage_type: StorageType,
    /// Listen to changes to this storage key from somewhere else. Defaults to true.
    pub(crate) listen_to_storage_changes: bool,
    /// If no value for the give key is found in the storage, write it. Defaults to true.
    pub(crate) write_defaults: bool,
    /// Takes the serialized (json) stored value and the default value and returns a merged version.
    /// Defaults to simply returning the stored value.
    pub(crate) merge_defaults: fn(&str, &T) -> String,
    /// Optional callback whenever an error occurs. The callback takes an argument of type [`UseStorageError`].
    pub(crate) on_error: Box<dyn CloneableFnWithArg<UseStorageError>>,

    /// Debounce or throttle the writing to storage whenever the value changes.
    pub(crate) filter: FilterOptions,
}

impl<T> Default for UseStorageOptions<T> {
    fn default() -> Self {
        Self {
            storage_type: Default::default(),
            listen_to_storage_changes: true,
            write_defaults: true,
            merge_defaults: |stored_value, _default_value| stored_value.to_string(),
            on_error: Box::new(|_| ()),
            filter: Default::default(),
        }
    }
}

impl<T> UseStorageOptions<T> {
    filter_builder_methods!(
        /// the serializing and storing into storage
        filter
    );
}
