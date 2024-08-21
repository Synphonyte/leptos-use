use cfg_if::cfg_if;
use js_sys::Function;
use std::ops::Deref;

use crate::core::impl_ssr_safe_method;
#[cfg(not(feature = "ssr"))]
use leptos::*;
use wasm_bindgen::JsValue;
use web_sys::{Document, Element, HtmlCollection, HtmlElement, HtmlHeadElement, Location, NodeList, VisibilityState, Window};

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
pub struct UseDocument(Option<Document>);

impl Deref for UseDocument {
    type Target = Option<Document>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UseDocument {
    impl_ssr_safe_method!(
        /// Returns `Some(Document)` in the Browser. `None` otherwise.
        body(&self) -> Option<HtmlElement>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        /// Returns the active (focused) `Some(web_sys::Element)` in the Browser. `None` otherwise.
        active_element(&self) -> Option<Element>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        query_selector(&self, selector: &str) -> Result<Option<Element>, JsValue>;
        .unwrap_or(Ok(None))
    );

    impl_ssr_safe_method!(
        query_selector_all(&self, selectors: &str) -> Option<Result<NodeList, JsValue>>
    );

    impl_ssr_safe_method!(
        url(&self) -> Option<Result<String, JsValue>>
    );

    impl_ssr_safe_method!(
        document_uri(&self) -> Option<Result<String, JsValue>>
    );

    impl_ssr_safe_method!(
        compat_mode(&self) -> Option<String>
    );

    impl_ssr_safe_method!(
        character_set(&self) -> Option<String>
    );

    impl_ssr_safe_method!(
        charset(&self) -> Option<String>
    );

    impl_ssr_safe_method!(
        input_encoding(&self) -> Option<String>
    );

    impl_ssr_safe_method!(
        content_type(&self) -> Option<String>
    );

    impl_ssr_safe_method!(
        document_element(&self) -> Option<Element>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        location(&self) -> Option<Location>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        referrer(&self) -> Option<String>
    );

    impl_ssr_safe_method!(
        last_modified(&self) -> Option<String>
    );

    impl_ssr_safe_method!(
        ready_state(&self) -> Option<String>
    );

    impl_ssr_safe_method!(
        title(&self) -> Option<String>
    );

    impl_ssr_safe_method!(
        dir(&self) -> Option<String>
    );

    impl_ssr_safe_method!(
        head(&self) -> Option<HtmlHeadElement>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        images(&self) -> Option<HtmlCollection>
    );

    impl_ssr_safe_method!(
        embeds(&self) -> Option<HtmlCollection>
    );

    impl_ssr_safe_method!(
        plugins(&self) -> Option<HtmlCollection>
    );

    impl_ssr_safe_method!(
        links(&self) -> Option<HtmlCollection>
    );

    impl_ssr_safe_method!(
        forms(&self) -> Option<HtmlCollection>
    );

    impl_ssr_safe_method!(
        scripts(&self) -> Option<HtmlCollection>
    );

    impl_ssr_safe_method!(
        default_view(&self) -> Option<Window>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        onreadystatechange(&self) -> Option<Function>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        onbeforescriptexecute(&self) -> Option<Function>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        onafterscriptexecute(&self) -> Option<Function>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        onselectionchange(&self) -> Option<Function>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        current_script(&self) -> Option<Element>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        anchors(&self) -> Option<HtmlCollection>
    );

    impl_ssr_safe_method!(
        applets(&self) -> Option<HtmlCollection>
    );

    impl_ssr_safe_method!(
        fullscreen(&self) -> Option<bool>
    );

    impl_ssr_safe_method!(
        fullscreen_enabled(&self) -> Option<bool>
    );

    impl_ssr_safe_method!(
        onfullscreenchange(&self) -> Option<Function>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        onfullscreenerror(&self) -> Option<Function>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        onpointerlockchange(&self) -> Option<Function>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        onpointerlockerror(&self) -> Option<Function>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        /// Hides on server by default
        hidden(&self) -> bool;
        .unwrap_or(true)
    );

    impl_ssr_safe_method!(
        visibility_state(&self) -> Option<VisibilityState>
    );

    impl_ssr_safe_method!(
        onvisibilitychange(&self) -> Option<Function>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        selected_style_sheet_set(&self) -> Option<String>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        last_style_sheet_set(&self) -> Option<String>;
        .unwrap_or_default()
    );

    impl_ssr_safe_method!(
        preferred_style_sheet_set(&self) -> Option<String>;
        .unwrap_or_default()
    );
}
