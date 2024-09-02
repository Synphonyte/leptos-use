use cfg_if::cfg_if;
use leptos::reactive_graph::wrappers::read::Signal;

/// Reactive [DeviceOrientationEvent](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent).
///
/// Provide web developers with information from the physical orientation of
/// the device running the web page.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_device_orientation)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_device_orientation, UseDeviceOrientationReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseDeviceOrientationReturn {
///     is_supported,
///     absolute,
///     alpha,
///     beta,
///     gamma,
/// } = use_device_orientation();
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this function returns values as if the orientation
/// capabilties were not supported by the device.
pub fn use_device_orientation() -> UseDeviceOrientationReturn {
    cfg_if! { if #[cfg(feature = "ssr")] {
        let is_supported = Signal::derive(|| false);
        let absolute = Signal::derive(|| false);
        let alpha = Signal::derive(|| None);
        let beta = Signal::derive(|| None);
        let gamma = Signal::derive(|| None);
    } else {
        use leptos::prelude::*;
        use crate::{use_event_listener_with_options, UseEventListenerOptions, use_supported, js};
        use leptos::ev::deviceorientation;
        use send_wrapper::SendWrapper;

        let is_supported = use_supported(|| js!("DeviceOrientationEvent" in &window()));
        let (absolute, set_absolute) = signal(false);
        let (alpha, set_alpha) = signal(None);
        let (beta, set_beta) = signal(None);
        let (gamma, set_gamma) = signal(None);

        if is_supported.get_untracked() {
            let cleanup = use_event_listener_with_options(
                window(),
                deviceorientation,
                move |event: web_sys::DeviceOrientationEvent| {
                    set_absolute.set(event.absolute());
                    set_alpha.set(event.alpha());
                    set_beta.set(event.beta());
                    set_gamma.set(event.gamma());
                },
                UseEventListenerOptions::default()
                    .capture(false)
                    .passive(true)
                    .once(false),
            );

            on_cleanup({
                let cleanup = SendWrapper::new(cleanup);
                #[allow(clippy::redundant_closure)]
                move || cleanup()
            });
        }
    }}

    #[allow(clippy::useless_conversion)]
    UseDeviceOrientationReturn {
        is_supported,
        absolute: absolute.into(),
        alpha: alpha.into(),
        beta: beta.into(),
        gamma: gamma.into(),
    }
}

/// Return type of [`use_device_orientation`].
#[derive(Clone)]
pub struct UseDeviceOrientationReturn {
    pub is_supported: Signal<bool>,
    pub absolute: Signal<bool>,
    pub alpha: Signal<Option<f64>>,
    pub beta: Signal<Option<f64>>,
    pub gamma: Signal<Option<f64>>,
}
