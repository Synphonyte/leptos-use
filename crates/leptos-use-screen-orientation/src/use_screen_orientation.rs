use crate::{ScreenOrientation, ScreenOrientationLock, UseScreenOrientationReturn};
use leptos::prelude::*;

/// Reactive [Screen Orientation API](https://developer.mozilla.org/en-US/docs/Web/API/Screen_Orientation_API).
/// It provides web developers with information about the user's current screen orientation.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_screen_orientation)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_screen_orientation::{use_screen_orientation, UseScreenOrientationReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseScreenOrientationReturn { orientation, angle, lock_orientation, unlock_orientation } = use_screen_orientation();
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server `orientation` will always be `ScreenOrientation::PortraitPrimary` and `angle` will always be `0`.
/// The locking functions will be no-ops.
pub fn use_screen_orientation() -> UseScreenOrientationReturn<
    impl Fn(ScreenOrientationLock) + Clone + Send + Sync + 'static,
    impl Fn() + Clone + Send + Sync + 'static,
> {
    #[cfg(feature = "ssr")]
    {
        UseScreenOrientationReturn {
            orientation: Signal::stored(ScreenOrientation::PortraitPrimary),
            angle: Signal::stored(0),
            lock_orientation: |_| {},
            unlock_orientation: || {},
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use std::rc::Rc;

        use crate::{
            UseEventListenerOptions, js_fut, sendwrap_fn, use_event_listener_with_options,
        };
        use leptos::ev::orientationchange;
        use leptos::logging::debug_warn;

        let screen_orientation = window()
            .screen()
            .ok()
            .map(|screen| Rc::new(screen.orientation()));

        let read_orientation = |screen_orientation: &web_sys::ScreenOrientation| {
            screen_orientation
                .type_()
                .map_or(ScreenOrientation::PortraitPrimary, Into::into)
        };

        let (orientation, set_orientation) = signal(
            screen_orientation
                .as_ref()
                .map_or(ScreenOrientation::PortraitPrimary, |screen_orientation| {
                    read_orientation(screen_orientation)
                }),
        );
        let (angle, set_angle) = signal(
            screen_orientation
                .as_ref()
                .and_then(|screen_orientation| screen_orientation.angle().ok())
                .unwrap_or_default(),
        );

        let _ = use_event_listener_with_options(
            window(),
            orientationchange,
            {
                let screen_orientation = screen_orientation.clone();

                move |_| {
                    if let Some(screen_orientation) = screen_orientation.as_ref() {
                        set_orientation.set(read_orientation(screen_orientation));
                        set_angle.set(screen_orientation.angle().unwrap_or_default());
                    }
                }
            },
            UseEventListenerOptions::default().passive(true),
        );

        let lock_orientation = {
            let screen_orientation = screen_orientation.clone();

            sendwrap_fn!(move |lock: ScreenOrientationLock| {
                let Some(screen_orientation) = screen_orientation.as_ref() else {
                    return;
                };

                match screen_orientation.lock(lock.into()) {
                    Ok(promise) => {
                        leptos::task::spawn_local(async move {
                            if let Err(err) = js_fut!(promise).await {
                                debug_warn!("cannot lock screen orientation: {:?}", err);
                            }
                        });
                    }
                    Err(err) => {
                        debug_warn!("cannot lock screen orientation: {:?}", err);
                    }
                }
            })
        };

        let unlock_orientation = sendwrap_fn!(move || {
            if let Some(screen_orientation) = screen_orientation.as_ref()
                && let Err(err) = screen_orientation.unlock()
            {
                debug_warn!("cannot unlock screen orientation: {:?}", err);
            }
        });

        UseScreenOrientationReturn {
            orientation: orientation.into(),
            angle: angle.into(),
            lock_orientation,
            unlock_orientation,
        }
    }
}
