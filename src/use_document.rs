use cfg_if::cfg_if;
use leptos::*;
use std::ops::Deref;

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
    /// Returns `Some(HtmlElement)` in the Browser. `None` otherwise.
    pub fn body(&self) -> Option<web_sys::HtmlElement> {
        self.0.as_ref().and_then(|d| d.body())
    }
}
