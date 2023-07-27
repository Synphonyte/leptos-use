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
/// # pub fn Demo() -> impl IntoView {
/// let is_supported = use_supported(
///     || JsValue::from("getBattery").js_in(&window().navigator())
/// );
///
/// if is_supported.get() {
///     // do something
/// }
/// #    view! { }
/// # }
/// ```
pub fn use_supported(callback: impl Fn() -> bool + 'static) -> Signal<bool> {
    Signal::derive(callback)
}
