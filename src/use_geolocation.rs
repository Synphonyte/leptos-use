use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;

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
/// # use leptos_use::{use_geolocation, UseGeolocationReturn};
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
/// ## Server-Side Rendering
///
/// On the server all signals returns will always contain `None` and the functions do nothing.
pub fn use_geolocation() -> UseGeolocationReturn<impl Fn() + Clone, impl Fn() + Clone> {
    use_geolocation_with_options(UseGeolocationOptions::default())
}

/// Version of [`use_geolocation`] that takes a `UseGeolocationOptions`. See [`use_geolocation`] for how to use.
pub fn use_geolocation_with_options(
    options: UseGeolocationOptions,
) -> UseGeolocationReturn<impl Fn() + Clone, impl Fn() + Clone> {
    let (located_at, set_located_at) = signal(None::<f64>);
    let (error, set_error) = signal_local(None::<web_sys::PositionError>);
    let (coords, set_coords) = signal_local(None::<web_sys::Coordinates>);

    cfg_if! { if #[cfg(feature = "ssr")] {
        let resume = || ();
        let pause = || ();

        let _ = options;
        let _ = set_located_at;
        let _ = set_error;
        let _ = set_coords;
    } else {
        use crate::use_window;
        use wasm_bindgen::prelude::*;
        use std::sync::{Arc, Mutex};

        let update_position = move |position: web_sys::Position| {
            set_located_at.set(Some(position.timestamp()));
            set_coords.set(Some(position.coords()));
            set_error.set(None);
        };

        let on_error = move |err: web_sys::PositionError| {
            set_error.set(Some(err));
        };

        let watch_handle = Arc::new(Mutex::new(None::<i32>));

        let resume = {
            let watch_handle = Arc::clone(&watch_handle);
            let position_options = options.as_position_options();

            move || {
                let navigator = use_window().navigator();
                if let Some(navigator) = navigator {
                    if let Ok(geolocation) = navigator.geolocation() {
                        let update_position =
                            Closure::wrap(Box::new(update_position) as Box<dyn Fn(web_sys::Position)>);
                        let on_error =
                            Closure::wrap(Box::new(on_error) as Box<dyn Fn(web_sys::PositionError)>);

                        *watch_handle.lock().unwrap() =
                            geolocation
                                .watch_position_with_error_callback_and_options(
                                    update_position.as_ref().unchecked_ref(),
                                    Some(on_error.as_ref().unchecked_ref()),
                                    &position_options,
                                )
                                .ok();

                        update_position.forget();
                        on_error.forget();
                    }
                }
            }
        };

        if options.immediate {
            resume();
        }

        let pause = {
            let watch_handle = Arc::clone(&watch_handle);

            move || {
                let navigator = use_window().navigator();
                if let Some(navigator) = navigator {
                    if let Some(handle) = *watch_handle.lock().unwrap() {
                        if let Ok(geolocation) = navigator.geolocation() {
                            geolocation.clear_watch(handle);
                        }
                    }
                }
            }
        };

        on_cleanup({
            let pause = pause.clone();

            move || {
                pause();
            }
        });
    }}

    UseGeolocationReturn {
        coords: coords.into(),
        located_at: located_at.into(),
        error: error.into(),
        resume,
        pause,
    }
}

/// Options for [`use_geolocation_with_options`].
#[derive(DefaultBuilder, Clone)]
#[allow(dead_code)]
pub struct UseGeolocationOptions {
    /// If `true` the geolocation watch is started when this function is called.
    /// If `false` you have to call `resume` manually to start it. Defaults to `true`.
    immediate: bool,

    /// A boolean value that indicates the application would like to receive the best
    /// possible results. If `true` and if the device is able to provide a more accurate
    /// position, it will do so. Note that this can result in slower response times or
    /// increased power consumption (with a GPS chip on a mobile device for example).
    /// On the other hand, if `false`, the device can take the liberty to save
    /// resources by responding more quickly and/or using less power. Default: `false`.
    enable_high_accuracy: bool,

    /// A positive value indicating the maximum age in milliseconds of a possible cached position that is acceptable to return.
    /// If set to `0`, it means that the device cannot use a cached position and must attempt to retrieve the real current position.
    /// Default: 30000.
    maximum_age: u32,

    /// A positive value representing the maximum length of time (in milliseconds)
    /// the device is allowed to take in order to return a position.
    /// The default value is 27000.
    timeout: u32,
}

impl Default for UseGeolocationOptions {
    fn default() -> Self {
        Self {
            enable_high_accuracy: false,
            maximum_age: 30000,
            timeout: 27000,
            immediate: true,
        }
    }
}

#[cfg(not(feature = "ssr"))]
impl UseGeolocationOptions {
    fn as_position_options(&self) -> web_sys::PositionOptions {
        let UseGeolocationOptions {
            enable_high_accuracy,
            maximum_age,
            timeout,
            ..
        } = self;

        let options = web_sys::PositionOptions::new();
        options.set_enable_high_accuracy(*enable_high_accuracy);
        options.set_maximum_age(*maximum_age);
        options.set_timeout(*timeout);

        options
    }
}

/// Return type of [`use_geolocation`].
pub struct UseGeolocationReturn<ResumeFn, PauseFn>
where
    ResumeFn: Fn() + Clone,
    PauseFn: Fn() + Clone,
{
    /// The coordinates of the current device like latitude and longitude.
    /// See [`GeolocationCoordinates`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates)..
    pub coords: Signal<Option<web_sys::Coordinates>, LocalStorage>,

    /// The timestamp of the current coordinates.
    pub located_at: Signal<Option<f64>>,

    /// The last error received from `navigator.geolocation`.
    pub error: Signal<Option<web_sys::PositionError>, LocalStorage>,

    /// Resume the geolocation watch.
    pub resume: ResumeFn,

    /// Pause the geolocation watch.
    pub pause: PauseFn,
}
