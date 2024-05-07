use leptos::prelude::*;

#[cfg_attr(feature = "ssr", allow(dead_code))]
fn get() -> web_sys::Url {
    web_sys::Url::new(
        &window()
            .location()
            .href()
            .expect("Failed to get location.href from the browser"),
    )
    .expect("Failed to parse location.href from the browser")
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
            current_url().search_params().get(k)
        }}
    }
}
