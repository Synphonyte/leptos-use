use crate::{use_document, UseDocument};
use cfg_if::cfg_if;
use leptos::*;
use std::ops::Deref;

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
/// # use leptos::*;
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
    /// Returns the `Some(Navigator)` in the Browser. `None` otherwise.
    pub fn navigator(&self) -> Option<web_sys::Navigator> {
        self.0.as_ref().map(|w| w.navigator())
    }

    /// Returns the same as [`use_document`].
    #[inline(always)]
    pub fn document(&self) -> UseDocument {
        use_document()
    }
}
