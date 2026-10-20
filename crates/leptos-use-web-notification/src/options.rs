use default_struct_builder::DefaultBuilder;
use std::rc::Rc;
use wasm_bindgen::JsValue;

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

/// Options for [`use_web_notification_with_options`](crate::use_web_notification_with_options).
/// See [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/API/notification) for more info.
///
/// The following implementations are missing:
#[derive(DefaultBuilder, Clone)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct UseWebNotificationOptions {
    /// The title property of the Notification interface indicates
    /// the title of the notification
    #[builder(into)]
    pub(crate) title: String,

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

    /// The URL of the image to be displayed as part of the notification as specified
    /// in the constructor's options parameter.
    #[builder(into)]
    image: Option<String>,

    /// A boolean value indicating that a notification should remain active until the
    /// user clicks or dismisses it, rather than closing automatically.
    require_interaction: bool,

    /// A boolean value specifying whether the user should be notified after a new notification replaces an old one.
    /// The default is `false`, which means they won't be notified. If `true`, then `tag` also must be set.
    #[builder(into)]
    renotify: bool,

    /// A boolean value specifying whether the notification should be silent, regardless of the device settings.
    /// The default is `null`, which means the notification is not silent. If `true`, then the notification will be silent.
    #[builder(into)]
    silent: Option<bool>,

    /// A `Vec<u16>` specifying the vibration pattern in milliseconds for vibrating and not vibrating.
    /// The last entry can be a vibration since it stops automatically after each period.
    #[builder(into)]
    vibrate: Option<Vec<u16>>,

    /// Called when the user clicks on displayed `Notification`.
    pub(crate) on_click: Rc<dyn Fn(web_sys::Event)>,

    /// Called when the user closes a `Notification`.
    pub(crate) on_close: Rc<dyn Fn(web_sys::Event)>,

    /// Called when something goes wrong with a `Notification`
    /// (in many cases an error preventing the notification from being displayed.)
    pub(crate) on_error: Rc<dyn Fn(web_sys::Event)>,

    /// Called when a `Notification` is displayed
    pub(crate) on_show: Rc<dyn Fn(web_sys::Event)>,
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
            image: None,
            require_interaction: false,
            renotify: false,
            silent: None,
            vibrate: None,
            on_click: Rc::new(|_| {}),
            on_close: Rc::new(|_| {}),
            on_error: Rc::new(|_| {}),
            on_show: Rc::new(|_| {}),
        }
    }
}

impl From<&UseWebNotificationOptions> for web_sys::NotificationOptions {
    fn from(options: &UseWebNotificationOptions) -> Self {
        let web_sys_options = Self::new();

        web_sys_options.set_dir(options.direction.into());
        web_sys_options.set_require_interaction(options.require_interaction);
        web_sys_options.set_renotify(options.renotify);
        web_sys_options.set_silent(options.silent);

        if let Some(body) = &options.body {
            web_sys_options.set_body(body);
        }

        if let Some(icon) = &options.icon {
            web_sys_options.set_icon(icon);
        }

        if let Some(image) = &options.image {
            web_sys_options.set_image(image);
        }

        if let Some(language) = &options.language {
            web_sys_options.set_lang(language);
        }

        if let Some(tag) = &options.tag {
            web_sys_options.set_tag(tag);
        }

        if let Some(vibrate) = &options.vibrate {
            web_sys_options.set_vibrate(&vibration_pattern_to_jsvalue(vibrate));
        }
        web_sys_options
    }
}

/// Options for [`UseWebNotificationReturn::show`](crate::UseWebNotificationReturn::show).
///
/// This can be used to override options passed to [`use_web_notification`](crate::use_web_notification).
/// See [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/API/notification) for more info.
///
/// The following implementations are missing:
#[derive(DefaultBuilder, Default)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct ShowOptions {
    /// The title property of the Notification interface indicates
    /// the title of the notification
    #[builder(into)]
    pub(crate) title: Option<String>,

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

    /// The URL of the image to be displayed as part of the notification as specified
    /// in the constructor's options parameter.
    #[builder(into)]
    image: Option<String>,

    /// A boolean value indicating that a notification should remain active until the
    /// user clicks or dismisses it, rather than closing automatically.
    #[builder(into)]
    require_interaction: Option<bool>,

    /// A boolean value specifying whether the user should be notified after a new notification replaces an old one.
    /// The default is `false`, which means they won't be notified. If `true`, then `tag` also must be set.
    #[builder(into)]
    renotify: Option<bool>,

    /// A boolean value specifying whether the notification should be silent, regardless of the device settings.
    /// The default is `null`, which means the notification is not silent. If `true`, then the notification will be silent.
    #[builder(into)]
    silent: Option<bool>,

    /// A `Vec<u16>` specifying the vibration pattern in milliseconds for vibrating and not vibrating.
    /// The last entry can be a vibration since it stops automatically after each period.
    #[builder(into)]
    vibrate: Option<Vec<u16>>,
}

#[cfg(not(feature = "ssr"))]
impl ShowOptions {
    pub(crate) fn override_notification_options(&self, options: &mut web_sys::NotificationOptions) {
        if let Some(direction) = self.direction {
            options.set_dir(direction.into());
        }

        if let Some(require_interaction) = self.require_interaction {
            options.set_require_interaction(require_interaction);
        }

        if let Some(body) = &self.body {
            options.set_body(body);
        }

        if let Some(icon) = &self.icon {
            options.set_icon(icon);
        }

        if let Some(image) = &self.image {
            options.set_image(image);
        }

        if let Some(language) = &self.language {
            options.set_lang(language);
        }

        if let Some(tag) = &self.tag {
            options.set_tag(tag);
        }

        if let Some(renotify) = self.renotify {
            options.set_renotify(renotify);
        }

        if let Some(silent) = self.silent {
            options.set_silent(Some(silent));
        }

        if let Some(vibrate) = &self.vibrate {
            options.set_vibrate(&vibration_pattern_to_jsvalue(vibrate));
        }
    }
}

/// Helper function to convert a slice of `u16` into a `JsValue` array that represents a vibration pattern
fn vibration_pattern_to_jsvalue(pattern: &[u16]) -> JsValue {
    let array = js_sys::Array::new();
    for &value in pattern.iter() {
        array.push(&JsValue::from(value));
    }
    array.into()
}
