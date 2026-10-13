use leptos::prelude::*;

/// SSR compatibe `is_supported`
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_supported::use_supported;
/// # use wasm_js_macros::js;
/// # use wasm_bindgen::JsValue;
/// #
/// # pub fn Demo() -> impl IntoView {
/// let is_supported = use_supported(
///     || js!("getBattery" in &window().navigator())
/// );
///
/// if is_supported.get() {
///     // do something
/// }
/// #    view! { }
/// # }
/// ```
pub fn use_supported(callback: impl Fn() -> bool + Send + Sync + 'static) -> Signal<bool> {
    #[cfg(feature = "ssr")]
    {
        let _ = callback;
        Signal::derive(|| false)
    }

    #[cfg(not(feature = "ssr"))]
    {
        let (supported, set_supported) = signal(false);

        Effect::new(move || set_supported.set(callback()));

        supported.into()
    }
}
