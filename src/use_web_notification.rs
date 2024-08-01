use crate::{use_supported, use_window};
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::*;
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
/// # use leptos::*;
/// # use leptos_use::{use_web_notification_with_options, UseWebNotificationOptions, ShowOptions, UseWebNotificationReturn, NotificationDirection};
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
///         .renotity(true)
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
/// This function is basically ignored on the server. You can safely call `show` but it will do nothing.
pub fn use_web_notification(
) -> UseWebNotificationReturn<impl Fn(ShowOptions) + Clone, impl Fn() + Clone> {
    use_web_notification_with_options(UseWebNotificationOptions::default())
}

/// Version of [`use_web_notification`] which takes an [`UseWebNotificationOptions`].
pub fn use_web_notification_with_options(
    options: UseWebNotificationOptions,
) -> UseWebNotificationReturn<impl Fn(ShowOptions) + Clone, impl Fn() + Clone> {
    let is_supported = use_supported(browser_supports_notifications);

    let (notification, set_notification) = create_signal(None::<web_sys::Notification>);

    let (permission, set_permission) = create_signal(NotificationPermission::default());

    cfg_if! { if #[cfg(feature = "ssr")] {
        let _ = options;
        let _ = set_notification;
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
                let prev = SpecialNonReactiveZone::enter();

                on_click(e);

                #[cfg(debug_assertions)]
                SpecialNonReactiveZone::exit(prev);
            }
        })
        .into_js_value();

        let on_close_closure = Closure::<dyn Fn(web_sys::Event)>::new({
            let on_close = Rc::clone(&options.on_close);
            move |e: web_sys::Event| {
                #[cfg(debug_assertions)]
                let prev = SpecialNonReactiveZone::enter();

                on_close(e);

                #[cfg(debug_assertions)]
                SpecialNonReactiveZone::exit(prev);
            }
        })
        .into_js_value();

        let on_error_closure = Closure::<dyn Fn(web_sys::Event)>::new({
            let on_error = Rc::clone(&options.on_error);
            move |e: web_sys::Event| {
                #[cfg(debug_assertions)]
                let prev = SpecialNonReactiveZone::enter();

                on_error(e);

                #[cfg(debug_assertions)]
                SpecialNonReactiveZone::exit(prev);
            }
        })
        .into_js_value();

        let on_show_closure = Closure::<dyn Fn(web_sys::Event)>::new({
            let on_show = Rc::clone(&options.on_show);
            move |e: web_sys::Event| {
                #[cfg(debug_assertions)]
                let prev = SpecialNonReactiveZone::enter();

                on_show(e);

                #[cfg(debug_assertions)]
                SpecialNonReactiveZone::exit(prev);
            }
        })
        .into_js_value();

        let show = {
            let options = options.clone();
            let on_click_closure = on_click_closure.clone();
            let on_close_closure = on_close_closure.clone();
            let on_error_closure = on_error_closure.clone();
            let on_show_closure = on_show_closure.clone();

            move |options_override: ShowOptions| {
                if !is_supported.get_untracked() {
                    return;
                }

                let options = options.clone();
                let on_click_closure = on_click_closure.clone();
                let on_close_closure = on_close_closure.clone();
                let on_error_closure = on_error_closure.clone();
                let on_show_closure = on_show_closure.clone();

                spawn_local(async move {
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

                    set_notification.set(Some(notification_value));
                });
            }
        };

        let close = {
            move || {
                notification.with_untracked(|notification| {
                    if let Some(notification) = notification {
                        notification.close();
                    }
                });
                set_notification.set(None);
            }
        };

        spawn_local(async move {
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
        notification: notification.into(),
        show,
        close,
        permission: permission.into(),
    }
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Debug)]
pub enum NotificationDirection {
    #[default]
    Auto,
    LeftToRight,
    RightToLeft,
}

impl From<NotificationDirection> for web_sys::NotificationDirection {
    fn from(direction: NotificationDirection) -> Self {
        match direction {
            NotificationDirection::Auto => Self::Auto,
            NotificationDirection::LeftToRight => Self::Ltr,
            NotificationDirection::RightToLeft => Self::Rtl,
        }
    }
}

/// Options for [`use_web_notification_with_options`].
/// See [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/API/notification) for more info.
///
/// The following implementations are missing:
/// - `vibrate`
/// - `image`
#[derive(DefaultBuilder, Clone)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct UseWebNotificationOptions {
    /// The title property of the Notification interface indicates
    /// the title of the notification
    #[builder(into)]
    title: String,

    /// The body string of the notification as specified in the constructor's
    /// options parameter.
    #[builder(into)]
    body: Option<String>,

    /// The text direction of the notification as specified in the constructor's
    /// options parameter. Can be `LeftToRight`, `RightToLeft` or `Auto` (default).
    /// See [`web_sys::NotificationDirection`] for more info.
    direction: NotificationDirection,

    /// The language code of the notification as specified in the constructor's
    /// options parameter.
    #[builder(into)]
    language: Option<String>,

    /// The ID of the notification(if any) as specified in the constructor's options
    /// parameter.
    #[builder(into)]
    tag: Option<String>,

    /// The URL of the image used as an icon of the notification as specified
    /// in the constructor's options parameter.
    #[builder(into)]
    icon: Option<String>,

    /// A boolean value indicating that a notification should remain active until the
    /// user clicks or dismisses it, rather than closing automatically.
    require_interaction: bool,

    /// A boolean value specifying whether the user should be notified after a new notification replaces an old one.
    /// The default is `false`, which means they won't be notified. If `true`, then `tag` also must be set.
    #[builder(into)]
    renotify: bool,

    /// A boolean value specifying whether the notification should be silent, regardless of the device settings.
    /// The default is `false`, which means the notification is not silent. If `true`, then the notification will be silent.
    #[builder(into)]
    silent: Option<bool>,

    /// Called when the user clicks on displayed `Notification`.
    on_click: Rc<dyn Fn(web_sys::Event)>,

    /// Called when the user closes a `Notification`.
    on_close: Rc<dyn Fn(web_sys::Event)>,

    /// Called when something goes wrong with a `Notification`
    /// (in many cases an error preventing the notification from being displayed.)
    on_error: Rc<dyn Fn(web_sys::Event)>,

    /// Called when a `Notification` is displayed
    on_show: Rc<dyn Fn(web_sys::Event)>,
}

impl Default for UseWebNotificationOptions {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            body: None,
            direction: NotificationDirection::default(),
            language: None,
            tag: None,
            icon: None,
            require_interaction: false,
            renotify: false,
            silent: None,
            on_click: Rc::new(|_| {}),
            on_close: Rc::new(|_| {}),
            on_error: Rc::new(|_| {}),
            on_show: Rc::new(|_| {}),
        }
    }
}

impl From<&UseWebNotificationOptions> for web_sys::NotificationOptions {
    fn from(options: &UseWebNotificationOptions) -> Self {
        let mut web_sys_options = Self::new();

        web_sys_options
            .dir(options.direction.into())
            .require_interaction(options.require_interaction)
            .renotify(options.renotify)
            .silent(options.silent);

        if let Some(body) = &options.body {
            web_sys_options.body(body);
        }

        if let Some(icon) = &options.icon {
            web_sys_options.icon(icon);
        }

        if let Some(language) = &options.language {
            web_sys_options.lang(language);
        }

        if let Some(tag) = &options.tag {
            web_sys_options.tag(tag);
        }

        web_sys_options
    }
}

/// Options for [`UseWebNotificationReturn::show`].
/// This can be used to override options passed to [`use_web_notification`].
/// See [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/API/notification) for more info.
///
/// The following implementations are missing:
/// - `vibrate`
/// - `image`
#[derive(DefaultBuilder, Default)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct ShowOptions {
    /// The title property of the Notification interface indicates
    /// the title of the notification
    #[builder(into)]
    title: Option<String>,

    /// The body string of the notification as specified in the constructor's
    /// options parameter.
    #[builder(into)]
    body: Option<String>,

    /// The text direction of the notification as specified in the constructor's
    /// options parameter. Can be `LeftToRight`, `RightToLeft` or `Auto` (default).
    /// See [`web_sys::NotificationDirection`] for more info.
    #[builder(into)]
    direction: Option<NotificationDirection>,

    /// The language code of the notification as specified in the constructor's
    /// options parameter.
    #[builder(into)]
    language: Option<String>,

    /// The ID of the notification(if any) as specified in the constructor's options
    /// parameter.
    #[builder(into)]
    tag: Option<String>,

    /// The URL of the image used as an icon of the notification as specified
    /// in the constructor's options parameter.
    #[builder(into)]
    icon: Option<String>,

    /// A boolean value indicating that a notification should remain active until the
    /// user clicks or dismisses it, rather than closing automatically.
    #[builder(into)]
    require_interaction: Option<bool>,

    /// A boolean value specifying whether the user should be notified after a new notification replaces an old one.
    /// The default is `false`, which means they won't be notified. If `true`, then `tag` also must be set.
    #[builder(into)]
    renotify: Option<bool>,

    /// A boolean value specifying whether the notification should be silent, regardless of the device settings.
    /// The default is `false`, which means the notification is not silent. If `true`, then the notification will be silent.
    #[builder(into)]
    silent: Option<bool>,
}

#[cfg(not(feature = "ssr"))]
impl ShowOptions {
    fn override_notification_options(&self, options: &mut web_sys::NotificationOptions) {
        if let Some(direction) = self.direction {
            options.dir(direction.into());
        }

        if let Some(require_interaction) = self.require_interaction {
            options.require_interaction(require_interaction);
        }

        if let Some(body) = &self.body {
            options.body(body);
        }

        if let Some(icon) = &self.icon {
            options.icon(icon);
        }

        if let Some(language) = &self.language {
            options.lang(language);
        }

        if let Some(tag) = &self.tag {
            options.tag(tag);
        }

        if let Some(renotify) = self.renotify {
            options.renotify(renotify);
        }

        if let Some(silent) = self.silent {
            options.silent(Some(silent));
        }
    }
}

/// Helper function to determine if browser supports notifications
fn browser_supports_notifications() -> bool {
    if let Some(window) = use_window().as_ref() {
        if window.has_own_property(&wasm_bindgen::JsValue::from_str("Notification")) {
            return true;
        }
    }

    false
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
/// The permission to send notifications
pub enum NotificationPermission {
    /// Notification has not been requested. In effect this is the same as `Denied`.
    #[default]
    Default,
    /// You are allowed to send notifications
    Granted,
    /// You are *not* allowed to send notifications
    Denied,
}

impl From<web_sys::NotificationPermission> for NotificationPermission {
    fn from(permission: web_sys::NotificationPermission) -> Self {
        match permission {
            web_sys::NotificationPermission::Default => Self::Default,
            web_sys::NotificationPermission::Granted => Self::Granted,
            web_sys::NotificationPermission::Denied => Self::Denied,
            web_sys::NotificationPermission::__Nonexhaustive => Self::Default,
        }
    }
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

/// Return type for [`use_web_notification`].
pub struct UseWebNotificationReturn<ShowFn, CloseFn>
where
    ShowFn: Fn(ShowOptions) + Clone,
    CloseFn: Fn() + Clone,
{
    pub is_supported: Signal<bool>,
    pub notification: Signal<Option<web_sys::Notification>>,
    pub show: ShowFn,
    pub close: CloseFn,
    pub permission: Signal<NotificationPermission>,
}
