use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::borrow::Cow;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::ServiceWorkerRegistration;

use crate::use_window;

///
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_service_worker)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_service_worker;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// #     let sw = use_service_worker_with_options(UseServiceWorkerOptions {
/// #         script_url: "service-worker.js".into(),
/// #         skip_waiting_message: "skipWaiting".into(),
/// #         ..UseServiceWorkerOptions::default()
/// #     });
/// #
/// #     view! { }
/// # }
/// ```
pub fn use_service_worker() -> UseServiceWorkerReturn {
    use_service_worker_with_options(UseServiceWorkerOptions::default())
}

/// Version of [`use_service_worker`] that takes a `UseServiceWorkerOptions`. See [`use_service_worker`] for how to use.
pub fn use_service_worker_with_options(options: UseServiceWorkerOptions) -> UseServiceWorkerReturn {
    // Reload the page whenever a new ServiceWorker is installed.
    if let Some(navigator) = use_window().navigator() {
        let on_controller_change = options.on_controller_change.clone();
        let reload = Closure::wrap(Box::new(move |_event: JsValue| {
            on_controller_change.call(());
        }) as Box<dyn FnMut(JsValue)>)
        .into_js_value();
        navigator
            .service_worker()
            .set_oncontrollerchange(Some(reload.as_ref().unchecked_ref()));
    }

    // Create async actions.
    let create_or_update_registration = create_action_create_or_update_sw_registration();
    let get_registration = create_action_get_sw_registration();
    let update_action = create_action_update_sw();

    // Immediately create or update the SW registration.
    create_or_update_registration.dispatch(ServiceWorkerScriptUrl(options.script_url.to_string()));

    // And parse the result into individual signals.
    let registration: Signal<Result<ServiceWorkerRegistration, ServiceWorkerRegistrationError>> =
        Signal::derive(move || {
            let a = get_registration.value().get();
            let b = create_or_update_registration.value().get();
            // We only dispatch create_or_update_registration once. Whenever we manually re-fetched the registration, the result of that has precedence!
            match a {
                Some(res) => res.map_err(ServiceWorkerRegistrationError::Js),
                None => match b {
                    Some(res) => res.map_err(ServiceWorkerRegistrationError::Js),
                    None => Err(ServiceWorkerRegistrationError::NeverQueried),
                },
            }
        });

    let fetch_registration = Closure::wrap(Box::new(move |_event: JsValue| {
        get_registration.dispatch(());
    }) as Box<dyn FnMut(JsValue)>)
    .into_js_value();

    // Handle a changing registration state.
    // Notify to developer if SW registration or retrieval fails.
    create_effect(move |_| {
        registration.with(|reg| match reg {
            Ok(registration) => {
                // We must be informed when an updated SW is available.
                registration.set_onupdatefound(Some(fetch_registration.as_ref().unchecked_ref()));

                // Trigger a check to see IF an updated SW is available.
                update_action.dispatch(registration.clone());

                // If a SW is installing, we must be notified if its state changes!
                if let Some(sw) = registration.installing() {
                    sw.set_onstatechange(Some(fetch_registration.as_ref().unchecked_ref()));
                }
            }
            Err(err) => match err {
                ServiceWorkerRegistrationError::Js(err) => {
                    tracing::warn!("ServiceWorker registration failed: {err:?}")
                }
                ServiceWorkerRegistrationError::NeverQueried => {}
            },
        })
    });

    UseServiceWorkerReturn {
        registration,
        installing: Signal::derive(move || {
            registration.with(|reg| {
                reg.as_ref()
                    .map(|reg| reg.installing().is_some())
                    .unwrap_or_default()
            })
        }),
        waiting: Signal::derive(move || {
            registration.with(|reg| {
                reg.as_ref()
                    .map(|reg| reg.waiting().is_some())
                    .unwrap_or_default()
            })
        }),
        active: Signal::derive(move || {
            registration.with(|reg| {
                reg.as_ref()
                    .map(|reg| reg.active().is_some())
                    .unwrap_or_default()
            })
        }),
        check_for_update: Callback::new(move |()| {
            registration.with(|reg| {
                if let Ok(reg) = reg {
                    update_action.dispatch(reg.clone())
                }
            })
        }),
        skip_waiting: Callback::new(move |()| {
            registration.with_untracked(|reg| if let Ok(reg) = reg {
                match reg.waiting() {
                    Some(sw) => {
                        tracing::info!("Updating to newly installed SW...");
                        sw.post_message(&JsValue::from_str(&options.skip_waiting_message)).expect("post message");
                    },
                    None => {
                        tracing::warn!("You tried to update the SW while no new SW was waiting. This is probably a bug.");
                    },
                }
            });
        }),
    }
}

/// Options for [`use_service_worker_with_options`].
#[derive(DefaultBuilder)]
pub struct UseServiceWorkerOptions {
    /// The name of your service-worker.
    /// You will most likely deploy the service-worker JS fiel alongside your app.
    /// A typical name is 'service-worker.js'.
    pub script_url: Cow<'static, str>,

    /// The message sent to a waiting ServiceWorker when you call the `skip_waiting` callback.
    pub skip_waiting_message: Cow<'static, str>,

    /// What should happen when a new service worker was activated?
    /// The default implementation reloads the current page.
    pub on_controller_change: Callback<()>,
}

impl Default for UseServiceWorkerOptions {
    fn default() -> Self {
        Self {
            script_url: "service-worker.js".into(),
            skip_waiting_message: "skipWaiting".into(),
            on_controller_change: Callback::new(move |()| {
                use std::ops::Deref;
                if let Some(window) = use_window().deref() {
                    match window.location().reload() {
                        Ok(()) => {}
                        Err(err) => tracing::warn!(
                            "Detected a ServiceWorkerController change but the page reload failed! Error: {err:?}"
                        ),
                    }
                }
            }),
        }
    }
}

/// Return type of [`use_service_worker`].
pub struct UseServiceWorkerReturn {
    /// The current registration state.
    pub registration: Signal<Result<ServiceWorkerRegistration, ServiceWorkerRegistrationError>>,

    /// Whether a SW is currently installing.
    pub installing: Signal<bool>,

    /// Whether a SW was installed and is now awaiting activation.
    pub waiting: Signal<bool>,

    /// Whether a SW is active.
    pub active: Signal<bool>,

    /// Check for ServiceWorker update.
    pub check_for_update: Callback<()>,

    /// Call this to activate a new ("waiting") SW if one is available.
    pub skip_waiting: Callback<()>,
}

struct ServiceWorkerScriptUrl(pub String);

#[derive(Debug, Clone)]
pub enum ServiceWorkerRegistrationError {
    Js(JsValue),
    NeverQueried,
}

/// A leptos action which asynchronously checks for ServiceWorker updates, given an existing ServiceWorkerRegistration.
fn create_action_update_sw(
) -> Action<ServiceWorkerRegistration, Result<ServiceWorkerRegistration, JsValue>> {
    create_action(move |registration: &ServiceWorkerRegistration| {
        let registration = registration.clone();
        async move {
            let update_promise = registration.update().expect("update to not fail");
            wasm_bindgen_futures::JsFuture::from(update_promise)
                .await
                .map(|ok| {
                    ok.dyn_into::<ServiceWorkerRegistration>()
                        .expect("conversion into ServiceWorkerRegistration")
                })
        }
    })
}

/// A leptos action which asynchronously creates or updates and than retrieves the ServiceWorkerRegistration.
fn create_action_create_or_update_sw_registration(
) -> Action<ServiceWorkerScriptUrl, Result<ServiceWorkerRegistration, JsValue>> {
    create_action(move |script_url: &ServiceWorkerScriptUrl| {
        let script_url = script_url.0.to_owned();
        async move {
            if let Some(navigator) = use_window().navigator() {
                let promise = navigator.service_worker().register(script_url.as_str());
                wasm_bindgen_futures::JsFuture::from(promise)
                    .await
                    .map(|ok| {
                        ok.dyn_into::<ServiceWorkerRegistration>()
                            .expect("conversion into ServiceWorkerRegistration")
                    })
            } else {
                Err(JsValue::from_str("no navigator"))
            }
        }
    })
}

/// A leptos action which asynchronously fetches the current ServiceWorkerRegistration.
fn create_action_get_sw_registration() -> Action<(), Result<ServiceWorkerRegistration, JsValue>> {
    create_action(move |(): &()| {
        async move {
            if let Some(navigator) = use_window().navigator() {
                let promise = navigator.service_worker().get_registration(); // Could take a scope like "/app"...
                wasm_bindgen_futures::JsFuture::from(promise)
                    .await
                    .map(|ok| {
                        ok.dyn_into::<ServiceWorkerRegistration>()
                            .expect("conversion into ServiceWorkerRegistration")
                    })
            } else {
                Err(JsValue::from_str("no navigator"))
            }
        }
    })
}
