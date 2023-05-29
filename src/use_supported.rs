use leptos::*;

/// SSR compatibe `is_supported`
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_supported;
/// # use wasm_bindgen::JsValue;
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// let is_supported = use_supported(
///     cx,
///     || JsValue::from("getBattery").js_in(&window().navigator())
/// );
///
/// if is_supported() {
///     // do something
/// }
/// #    view! { cx, }
/// # }
/// ```
pub fn use_supported(cx: Scope, callback: impl Fn() -> bool + 'static) -> Signal<bool> {
    Signal::derive(cx, callback)
}
