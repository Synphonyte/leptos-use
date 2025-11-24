use leptos::prelude::*;

/// SSR compatibe `is_supported`
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_supported, js};
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
        // make sure we do not create hydration errors by calling the callback, when the client is mounted.

        // 1. create a signal that tracks if we are mounted
        let (is_mounted, set_mounted) = signal(false);

        // 2. create an effect that sets is_mounted to true on mount
        Effect::new(move |_| set_mounted.set(true));

        // 3. create a derived signal that calls the callback only when mounted
        Signal::derive(move || if is_mounted.get() { callback() } else { false })
    }
}
