use cfg_if::cfg_if;
use leptos::prelude::*;

/// Reactive [`window.devicePixelRatio`](https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio)
///
/// > NOTE: there is no event listener for `window.devicePixelRatio` change.
/// > So this function uses the same mechanism as described in
/// > [this example](https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio#monitoring_screen_resolution_or_zoom_level_changes).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_device_pixel_ratio)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_device_pixel_ratio;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let pixel_ratio = use_device_pixel_ratio();
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this function returns a Signal that is always `1.0`.
pub fn use_device_pixel_ratio() -> Signal<f64> {
    cfg_if! { if #[cfg(feature = "ssr")] {
        Signal::derive(|| 1.0)
    } else {
        use crate::{use_event_listener_with_options, UseEventListenerOptions};

        use leptos::ev::change;

        let initial_pixel_ratio = window().device_pixel_ratio();
        let (pixel_ratio, set_pixel_ratio) = signal(initial_pixel_ratio);

        Effect::new(move |_| {
            let media = window().match_media(
                &format!("(resolution: {}dppx)", pixel_ratio.get())
            ).unwrap();

            _ = use_event_listener_with_options(
                media,
                change,
                move |_| {
                    set_pixel_ratio.set(window().device_pixel_ratio());
                },
                UseEventListenerOptions::default()
                    .capture(false)
                    .passive(true)
                    .once(true),
            );
        });

        pixel_ratio.into()
    }}
}
