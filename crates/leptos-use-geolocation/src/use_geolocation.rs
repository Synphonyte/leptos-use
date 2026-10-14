use crate::{OptionLocalRwSignal, UseGeolocationOptions, UseGeolocationReturn};
use leptos::prelude::*;

/// Reactive [Geolocation API](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation_API).
///
/// It allows the user to provide their location to web applications if they so desire. For privacy reasons,
/// the user is asked for permission to report location information.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_geolocation)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_geolocation::{use_geolocation, UseGeolocationReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseGeolocationReturn {
///     coords,
///     located_at,
///     error,
///     resume,
///     pause,
/// } = use_geolocation();
/// #
/// # view! { }
/// # }
/// ```
///
///
/// ## SendWrapped Return
///
/// The returned closures `pause` and `resume` are sendwrapped functions. They can
/// only be called from the same thread that called `use_geolocation`.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server all signals returns will always contain `None` and the functions do nothing.
pub fn use_geolocation()
-> UseGeolocationReturn<impl Fn() + Clone + Send + Sync, impl Fn() + Clone + Send + Sync> {
    use_geolocation_with_options(UseGeolocationOptions::default())
}

/// Version of [`use_geolocation`] that takes a `UseGeolocationOptions`. See [`use_geolocation`] for how to use.
pub fn use_geolocation_with_options(
    options: UseGeolocationOptions,
) -> UseGeolocationReturn<impl Fn() + Clone + Send + Sync, impl Fn() + Clone + Send + Sync> {
    let (located_at, set_located_at) = signal(None::<f64>);
    let error = OptionLocalRwSignal::<web_sys::PositionError>::new();
    let coords = OptionLocalRwSignal::<web_sys::Coordinates>::new();

    let resume;
    let pause;

    #[cfg(feature = "ssr")]
    {
        resume = || ();
        pause = || ();

        let _ = options;
        let _ = set_located_at;
        let _ = error;
        let _ = coords;
    }

    #[cfg(not(feature = "ssr"))]
    {
        use crate::{sendwrap_fn, use_window};
        use std::sync::{Arc, Mutex};
        use wasm_bindgen::prelude::*;

        let update_position = move |position: web_sys::Position| {
            set_located_at.set(Some(position.timestamp()));
            coords.set(Some(position.coords()));
            error.set(None);
        };

        let on_error = move |err: web_sys::PositionError| {
            error.set(Some(err));
        };

        let watch_handle = Arc::new(Mutex::new(None::<i32>));

        resume = {
            let watch_handle = Arc::clone(&watch_handle);
            let position_options = options.as_position_options();

            sendwrap_fn!(move || {
                let navigator = use_window().navigator();
                if let Some(navigator) = navigator
                    && let Ok(geolocation) = navigator.geolocation()
                {
                    if let Some(handle) = watch_handle.lock().unwrap().take() {
                        geolocation.clear_watch(handle);
                    }

                    let update_position =
                        Closure::wrap(Box::new(update_position) as Box<dyn Fn(web_sys::Position)>);
                    let on_error =
                        Closure::wrap(Box::new(on_error) as Box<dyn Fn(web_sys::PositionError)>);

                    let handle = geolocation.watch_position_with_error_callback_and_options(
                        update_position.as_ref().unchecked_ref(),
                        Some(on_error.as_ref().unchecked_ref()),
                        &position_options,
                    );

                    #[cfg(web_sys_unstable_apis)]
                    let handle = Some(handle);

                    #[cfg(not(web_sys_unstable_apis))]
                    let handle = handle.ok();

                    *watch_handle.lock().unwrap() = handle;

                    update_position.forget();
                    on_error.forget();
                }
            })
        };

        if options.immediate {
            resume();
        }

        pause = {
            let watch_handle = Arc::clone(&watch_handle);

            sendwrap_fn!(move || {
                let navigator = use_window().navigator();
                if let Some(navigator) = navigator
                    && let Some(handle) = *watch_handle.lock().unwrap()
                    && let Ok(geolocation) = navigator.geolocation()
                {
                    geolocation.clear_watch(handle);
                }
            })
        };

        on_cleanup({
            let pause = pause.clone();

            move || {
                pause();
            }
        });
    }

    UseGeolocationReturn {
        coords: coords.read_only(),
        located_at: located_at.into(),
        error: error.read_only(),
        resume,
        pause,
    }
}
