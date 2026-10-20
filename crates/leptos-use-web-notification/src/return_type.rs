use crate::{OptionLocalSignal, ShowOptions};
use leptos::reactive::wrappers::read::Signal;

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
            _ => Self::Default,
        }
    }
}

/// Return type for [`use_web_notification`](crate::use_web_notification).
pub struct UseWebNotificationReturn<ShowFn, CloseFn>
where
    ShowFn: Fn(ShowOptions) + Clone + Send + Sync,
    CloseFn: Fn() + Clone + Send + Sync,
{
    pub is_supported: Signal<bool>,
    pub notification: OptionLocalSignal<web_sys::Notification>,
    pub show: ShowFn,
    pub close: CloseFn,
    pub permission: Signal<NotificationPermission>,
}
