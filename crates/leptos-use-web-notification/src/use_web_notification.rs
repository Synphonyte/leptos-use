use crate::{
    NotificationPermission, OptionLocalRwSignal, ShowOptions, UseWebNotificationOptions,
    UseWebNotificationReturn, use_supported, use_window,
};
use cfg_if::cfg_if;
use leptos::prelude::*;
#[cfg(not(feature = "ssr"))]
use std::rc::Rc;

/// Reactive [Notification API](https://developer.mozilla.org/en-US/docs/Web/API/Notification).
///
/// The Web Notification interface of the Notifications API is used to configure and display desktop notifications to the user.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_web_notification)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_web_notification::{use_web_notification_with_options, UseWebNotificationOptions, ShowOptions, UseWebNotificationReturn, NotificationDirection};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseWebNotificationReturn {
///     show,
///     close,
///     ..
/// } = use_web_notification_with_options(
///     UseWebNotificationOptions::default()
///         .direction(NotificationDirection::Auto)
///         .language("en")
///         .renotify(true)
///         .tag("test"),
/// );
///
/// show(ShowOptions::default().title("Hello World from leptos-use"));
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// This function is basically ignored on the server. You can safely call `show` but it will do nothing.
pub fn use_web_notification() -> UseWebNotificationReturn<
    impl Fn(ShowOptions) + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
> {
    use_web_notification_with_options(UseWebNotificationOptions::default())
}

/// Version of [`use_web_notification`] which takes an [`UseWebNotificationOptions`].
pub fn use_web_notification_with_options(
    options: UseWebNotificationOptions,
) -> UseWebNotificationReturn<
    impl Fn(ShowOptions) + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
> {
    let is_supported = use_supported(browser_supports_notifications);

    let notification = OptionLocalRwSignal::<web_sys::Notification>::new();

    let (permission, set_permission) = signal(NotificationPermission::default());

    cfg_if! { if #[cfg(feature = "ssr")] {
        let _ = options;
        let _ = notification;
        let _ = set_permission;

        let show = move |_: ShowOptions| ();
        let close = move || ();
    } else {
        use crate::use_event_listener;
        use leptos::ev::visibilitychange;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        let on_click_closure = Closure::<dyn Fn(web_sys::Event)>::new({
            let on_click = Rc::clone(&options.on_click);
            move |e: web_sys::Event| {
                #[cfg(debug_assertions)]
                let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                on_click(e);
            }
        })
        .into_js_value();

        let on_close_closure = Closure::<dyn Fn(web_sys::Event)>::new({
            let on_close = Rc::clone(&options.on_close);
            move |e: web_sys::Event| {
                #[cfg(debug_assertions)]
                let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                on_close(e);
            }
        })
        .into_js_value();

        let on_error_closure = Closure::<dyn Fn(web_sys::Event)>::new({
            let on_error = Rc::clone(&options.on_error);
            move |e: web_sys::Event| {
                #[cfg(debug_assertions)]
                let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                on_error(e);
            }
        })
        .into_js_value();

        let on_show_closure = Closure::<dyn Fn(web_sys::Event)>::new({
            let on_show = Rc::clone(&options.on_show);
            move |e: web_sys::Event| {
                #[cfg(debug_assertions)]
                let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                on_show(e);
            }
        })
        .into_js_value();

        let show = {
            let options = options.clone();
            let on_click_closure = on_click_closure.clone();
            let on_close_closure = on_close_closure.clone();
            let on_error_closure = on_error_closure.clone();
            let on_show_closure = on_show_closure.clone();

            let show = move |options_override: ShowOptions| {
                if !is_supported.get_untracked() {
                    return;
                }

                let options = options.clone();
                let on_click_closure = on_click_closure.clone();
                let on_close_closure = on_close_closure.clone();
                let on_error_closure = on_error_closure.clone();
                let on_show_closure = on_show_closure.clone();

                leptos::task::spawn_local(async move {
                    set_permission.set(request_web_notification_permission().await);

                    let mut notification_options = web_sys::NotificationOptions::from(&options);
                    options_override.override_notification_options(&mut notification_options);

                    let notification_value = web_sys::Notification::new_with_options(
                        &options_override.title.unwrap_or(options.title),
                        &notification_options,
                    )
                    .expect("Notification should be created");

                    notification_value.set_onclick(Some(on_click_closure.unchecked_ref()));
                    notification_value.set_onclose(Some(on_close_closure.unchecked_ref()));
                    notification_value.set_onerror(Some(on_error_closure.unchecked_ref()));
                    notification_value.set_onshow(Some(on_show_closure.unchecked_ref()));

                    notification.set(Some(notification_value));
                });
            };
            let wrapped_show = send_wrapper::SendWrapper::new(show);
            move |options_override: ShowOptions| wrapped_show(options_override)
        };

        let close = {
            move || {
                notification.with_untracked(|notification| {
                    if let Some(notification) = notification {
                        notification.close();
                    }
                });
                notification.set(None);
            }
        };

        leptos::task::spawn_local(async move {
            set_permission.set(request_web_notification_permission().await);
        });

        on_cleanup(close);

        // Use close() to remove a notification that is no longer relevant to to
        // the user (e.g.the user already read the notification on the webpage).
        // Most modern browsers dismiss notifications automatically after a few
        // moments(around four seconds).
        if is_supported.get_untracked() {
            let _ = use_event_listener(document(), visibilitychange, move |e: web_sys::Event| {
                e.prevent_default();
                if document().visibility_state() == web_sys::VisibilityState::Visible {
                    // The tab has become visible so clear the now-stale Notification:
                    close()
                }
            });
        }
    }}

    UseWebNotificationReturn {
        is_supported,
        notification: notification.read_only(),
        show,
        close,
        permission: permission.into(),
    }
}

/// Helper function to determine if browser supports notifications
fn browser_supports_notifications() -> bool {
    if let Some(window) = use_window().as_ref()
        && js_sys::Object::has_own(window, &wasm_bindgen::JsValue::from_str("Notification"))
    {
        return true;
    }

    false
}

/// Use `window.Notification.requestPosition()`. Returns a future that should be awaited
/// at least once before using [`use_web_notification`] to make sure
/// you have the permission to send notifications.
#[cfg(not(feature = "ssr"))]
async fn request_web_notification_permission() -> NotificationPermission {
    if let Ok(notification_permission) = web_sys::Notification::request_permission() {
        let _ = crate::js_fut!(notification_permission).await;
    }

    web_sys::Notification::permission().into()
}
