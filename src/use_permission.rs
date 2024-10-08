use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::fmt::Display;

/// Reactive [Permissions API](https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_permission)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_permission;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let microphone_access = use_permission("microphone");
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server the returned signal will always be `PermissionState::Unknown`.
pub fn use_permission(permission_name: &str) -> Signal<PermissionState> {
    let (state, set_state) = signal(PermissionState::Unknown);

    #[cfg(not(feature = "ssr"))]
    {
        use crate::use_event_listener;
        use std::cell::RefCell;
        use std::rc::Rc;

        let permission_status = Rc::new(RefCell::new(None::<web_sys::PermissionStatus>));

        let on_change = {
            let permission_status = Rc::clone(&permission_status);

            move || {
                if let Some(permission_status) = permission_status.borrow().as_ref() {
                    set_state.set(PermissionState::from(permission_status.state()));
                }
            }
        };

        leptos::task::spawn_local({
            let permission_name = permission_name.to_owned();

            async move {
                if let Ok(status) = query_permission(permission_name).await {
                    let _ = use_event_listener(status.clone(), leptos::ev::change, {
                        let on_change = on_change.clone();
                        move |_| on_change()
                    });
                    permission_status.replace(Some(status));
                    on_change();
                } else {
                    set_state.set(PermissionState::Prompt);
                }
            }
        });
    }

    #[cfg(feature = "ssr")]
    {
        let _ = set_state;
        let _ = permission_name;
    }

    state.into()
}

/// Return type of [`use_permission`].
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum PermissionState {
    /// State hasn't been requested yet. This is the initial value.
    #[default]
    Unknown,

    /// The permission has been granted by the user.
    Granted,

    /// The user will automatically be prompted to give permission once the relevant API is called.
    Prompt,

    /// The user has denied permission.
    Denied,
}

impl Display for PermissionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PermissionState::Unknown => write!(f, "unknown"),
            PermissionState::Granted => write!(f, "granted"),
            PermissionState::Prompt => write!(f, "prompt"),
            PermissionState::Denied => write!(f, "denied"),
        }
    }
}

impl From<web_sys::PermissionState> for PermissionState {
    fn from(permission_state: web_sys::PermissionState) -> Self {
        match permission_state {
            web_sys::PermissionState::Granted => PermissionState::Granted,
            web_sys::PermissionState::Prompt => PermissionState::Prompt,
            web_sys::PermissionState::Denied => PermissionState::Denied,
            _ => PermissionState::Unknown,
        }
    }
}

#[cfg(not(feature = "ssr"))]
async fn query_permission(
    permission: String,
) -> Result<web_sys::PermissionStatus, wasm_bindgen::JsValue> {
    use crate::{js, js_fut};
    use wasm_bindgen::JsCast;

    let permission_object = js_sys::Object::new();
    js!(permission_object["name"] = permission);

    let permission_state: web_sys::PermissionStatus = js_fut!(window()
        .navigator()
        .permissions()?
        .query(&permission_object)?)
    .await?
    .unchecked_into();

    Ok(permission_state)
}
