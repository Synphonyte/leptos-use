use cfg_if::cfg_if;
use std::ops::Deref;

use crate::core::impl_ssr_safe_method;
#[cfg(not(feature = "ssr"))]
use leptos::*;

/// SSR safe `document()`.
/// This returns just a new-type wrapper around `Option<Document>`.
/// Calling this amounts to `None` on the server and `Some(Document)` on the client.
///
/// It provides some convenient methods for working with the document like `body()`.
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_document;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let document = use_document();
///
/// // Returns `None` on the server but will not panic.
/// let body = document.body();
/// #
/// # view! { }
/// # }
/// ```
pub fn use_document() -> UseDocument {
    cfg_if! { if #[cfg(feature = "ssr")] {
        UseDocument(None)
    } else {
        UseDocument(Some(document()))
    }}
}

/// Return type of [`use_document`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseDocument(Option<web_sys::Document>);

impl Deref for UseDocument {
    type Target = Option<web_sys::Document>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UseDocument {
    impl_ssr_safe_method!(
        /// Returns `Some(Document)` in the Browser. `None` otherwise.
        body(&self) -> Option<web_sys::HtmlElement>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        /// Returns the active (focused) `Some(web_sys::Element)` in the Browser. `None` otherwise.
        active_element(&self) -> Option<web_sys::Element>;
        .unwrap_or_default()
    );
}
