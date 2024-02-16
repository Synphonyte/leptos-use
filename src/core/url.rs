use leptos::window;

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
    use super::get as current_url;

    /// Get a URL param value from the URL of the browser
    pub fn get(k: &str) -> Option<String> {
        current_url().search_params().get(k)
    }
}
