#[cfg(not(feature = "ssr"))]
use crate::MeasureType;
use crate::{
    UseEventListenerOptions, UseWindowSizeOptions, UseWindowSizeReturn,
    use_event_listener_with_options, use_media_query, use_window,
};
use leptos::ev::resize;
use leptos::prelude::*;

/// Reactive window size.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_window_size)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use_window_size::{use_window_size, UseWindowSizeReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseWindowSizeReturn { width, height } = use_window_size();
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server the width and height are always `initial_size` which defaults to
/// `Size { width: INFINITY, height: INFINITY }`.
pub fn use_window_size() -> UseWindowSizeReturn {
    use_window_size_with_options(UseWindowSizeOptions::default())
}

/// Version of [`fn@crate::use_window_size`] that takes a `UseWindowSizeOptions`. See [`fn@crate::use_window_size`] for how to use.
pub fn use_window_size_with_options(options: UseWindowSizeOptions) -> UseWindowSizeReturn {
    let UseWindowSizeOptions {
        initial_size,
        listen_orientation,
        include_scrollbar,
        measure_type,
    } = options;

    let (width, set_width) = signal(initial_size.width);
    let (height, set_height) = signal(initial_size.height);

    let update;

    #[cfg(not(feature = "ssr"))]
    {
        update = move || match measure_type {
            MeasureType::Outer => {
                set_width.set(
                    window()
                        .outer_width()
                        .expect("failed to get window width")
                        .as_f64()
                        .expect("width is not a f64"),
                );
                set_height.set(
                    window()
                        .outer_height()
                        .expect("failed to get window height")
                        .as_f64()
                        .expect("height is not a f64"),
                );
            }
            MeasureType::Inner => {
                if include_scrollbar {
                    set_width.set(
                        window()
                            .inner_width()
                            .expect("failed to get window width")
                            .as_f64()
                            .expect("width is not a f64"),
                    );
                    set_height.set(
                        window()
                            .inner_height()
                            .expect("failed to get window height")
                            .as_f64()
                            .expect("height is not a f64"),
                    );
                } else {
                    set_width.set(
                        document()
                            .document_element()
                            .expect("no document element")
                            .client_width() as f64,
                    );
                    set_height.set(
                        document()
                            .document_element()
                            .expect("no document element")
                            .client_height() as f64,
                    );
                }
            }
        };
    }

    #[cfg(feature = "ssr")]
    {
        update = || {};

        let _ = initial_size;
        let _ = include_scrollbar;
        let _ = measure_type;

        let _ = set_width;
        let _ = set_height;
    }

    update();
    let _ = use_event_listener_with_options(
        use_window(),
        resize,
        move |_| update(),
        UseEventListenerOptions::default().passive(true),
    );

    if listen_orientation {
        let matches = use_media_query("(orientation: portrait)");

        Effect::new(move |_| {
            let _ = matches.get();

            update();
        });
    }

    UseWindowSizeReturn {
        width: width.into(),
        height: height.into(),
    }
}
