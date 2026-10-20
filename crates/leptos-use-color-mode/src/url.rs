use leptos::prelude::*;

#[cfg_attr(feature = "ssr", allow(dead_code))]
/// Returns `None` if `location.href` is inaccessible (for example in a sandboxed
/// cross-origin iframe, where it throws a `SecurityError`) or cannot be parsed.
fn get() -> Option<web_sys::Url> {
    let href = window().location().href().ok()?;
    web_sys::Url::new(&href).ok()
}

pub mod params {
    use cfg_if::cfg_if;

    /// Get a URL param value from the URL of the browser
    pub fn get(k: &str) -> Option<String> {
        cfg_if! { if #[cfg(feature = "ssr")] {
            _ = k;
            None
        } else {
            use super::get as current_url;
            current_url()?.search_params().get(k)
        }}
    }
}
