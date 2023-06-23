use leptos::window;
use wasm_bindgen::JsValue;

/// Local or session storage or a custom store that is a `web_sys::Storage`.
#[doc(cfg(feature = "storage"))]
#[derive(Default)]
pub enum StorageType {
    #[default]
    Local,
    Session,
    Custom(web_sys::Storage),
}

impl StorageType {
    pub fn into_storage(self) -> Result<Option<web_sys::Storage>, JsValue> {
        match self {
            StorageType::Local => window().local_storage(),
            StorageType::Session => window().session_storage(),
            StorageType::Custom(storage) => Ok(Some(storage)),
        }
    }
}
