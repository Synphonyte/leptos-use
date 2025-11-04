use crate::core::impl_ssr_safe_method;
use crate::{UseDocument, use_document};
use cfg_if::cfg_if;
use std::ops::Deref;

#[cfg(not(feature = "ssr"))]
use leptos::prelude::*;

/// SSR safe `window()`.
/// This returns just a new-type wrapper around `Option<Window>`.
/// Calling this amounts to `None` on the server and `Some(Window)` on the client.
///
/// It provides some convenient methods for working with the window like `document()` and `navigator()`.
/// These will all return `None` on the server.
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_window;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let window = use_window();
///
/// // Returns `None` on the server but will not panic.
/// let navigator = window.navigator();
/// #
/// # view! { }
/// # }
/// ```
pub fn use_window() -> UseWindow {
    cfg_if! { if #[cfg(feature = "ssr")] {
        UseWindow(None)
    } else {
        UseWindow(Some(window()))
    }}
}

/// Return type of [`use_window`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseWindow(Option<web_sys::Window>);

impl Deref for UseWindow {
    type Target = Option<web_sys::Window>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UseWindow {
    impl_ssr_safe_method!(
        /// Returns `Some(Navigator)` in the Browser. `None` otherwise.
        navigator(&self) -> Option<web_sys::Navigator>
    );

    /// Returns the same as [`fn@use_document`].
    #[inline(always)]
    pub fn document(&self) -> UseDocument {
        use_document()
    }

    impl_ssr_safe_method!(
        /// Returns the same as `window().match_media()` in the Browser. `Ok(None)` otherwise.
        match_media(&self, query: &str) -> Result<Option<web_sys::MediaQueryList>, wasm_bindgen::JsValue>;
        .unwrap_or(Ok(None))
    );
}
