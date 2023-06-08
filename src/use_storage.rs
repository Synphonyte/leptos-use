use default_struct_builder::DefaultBuilder;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

// pub fn use_storage_with_options<T, D, MergeFn, ErrorFn>(
//     cx: Scope,
//     key: &str,
//     defaults: D,
//     options: UseStorageOptions<T, MergeFn, ErrorFn>,
// ) -> (ReadSignal<T>, WriteSignal<T>)
// where
//     for<'de> T: Serialize + Deserialize<'de> + Clone + 'static,
//     D: Into<MaybeSignal<T>>,
//     MergeFn: Fn(T, T) -> T,
//     ErrorFn: Fn(JsValue) + Clone,
// {
//     let defaults = defaults.into();
//
//     let (data, set_data) = create_signal(cx, defaults.get_untracked());
//
//     let storage = match options.storage_type {
//         StorageType::Local => window().local_storage(),
//         StorageType::Session => window().session_storage(),
//     };
//
//     match storage {
//         Ok(Some(storage)) => {}
//         Err(e) => options.on_error(e),
//         _ => {
//             // do nothing
//         }
//     }
//
//     (data, set_data)
// }

#[derive(DefaultBuilder)]
pub struct UseStorageOptions<T, MergeFn, ErrorFn>
where
    MergeFn: Fn(T, T) -> T,
    ErrorFn: Fn(JsValue),
    for<'de> T: Serialize + Deserialize<'de> + 'static,
{
    storage_type: StorageType,
    listen_to_storage_changes: bool,
    write_defaults: bool,
    merge_defaults: MergeFn,
    on_error: ErrorFn,

    _marker: std::marker::PhantomData<T>,
}

#[derive(Default)]
pub enum StorageType {
    #[default]
    Local,
    Session,
}
