use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos::reactive::actions::Action;
use leptos::reactive::wrappers::read::Signal;
use send_wrapper::SendWrapper;
use std::sync::Arc;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::ServiceWorkerRegistration;

use crate::{js_fut, use_window};

/// Reactive [ServiceWorker API](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API).
///
/// Please check the [working example](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_service_worker).
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_service_worker_with_options, UseServiceWorkerOptions, UseServiceWorkerReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseServiceWorkerReturn {
///         registration,
///         installing,
///         waiting,
///         active,
///         skip_waiting,
///         check_for_update,
/// } = use_service_worker_with_options(UseServiceWorkerOptions::default()
///     .script_url("service-worker.js")
///     .skip_waiting_message("skipWaiting"),
/// );
///
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// This function does **not** support SSR. Call it inside a `create_effect`.
pub fn use_service_worker() -> UseServiceWorkerReturn<impl Fn() + Clone, impl Fn() + Clone> {
    use_service_worker_with_options(UseServiceWorkerOptions::default())
}

/// Version of [`use_service_worker`] that takes a `UseServiceWorkerOptions`. See [`use_service_worker`] for how to use.
pub fn use_service_worker_with_options(
    options: UseServiceWorkerOptions,
) -> UseServiceWorkerReturn<impl Fn() + Clone, impl Fn() + Clone> {
    // Trigger the user-defined action (page-reload by default)
    // whenever a new ServiceWorker is installed.
    if let Some(navigator) = use_window().navigator() {
        let on_controller_change = options.on_controller_change.clone();
        let js_closure = Closure::wrap(Box::new(move |_event: JsValue| {
            #[cfg(debug_assertions)]
            let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

            on_controller_change();
        }) as Box<dyn FnMut(JsValue)>)
        .into_js_value();
        navigator
            .service_worker()
            .set_oncontrollerchange(Some(js_closure.as_ref().unchecked_ref()));
    }

    // Create async actions.
    let create_or_update_registration = create_action_create_or_update_registration();
    let get_registration = create_action_get_registration();
    let update_sw = create_action_update();

    // Immediately create or update the SW registration.
    create_or_update_registration.dispatch(ServiceWorkerScriptUrl(options.script_url.to_string()));

    // And parse the result into individual signals.
    let registration: Signal<
        Result<SendWrapper<ServiceWorkerRegistration>, ServiceWorkerRegistrationError>,
    > = Signal::derive(move || {
        let a = get_registration.value().get();
        let b = create_or_update_registration.value().get();
        // We only dispatch create_or_update_registration once.
        // Whenever we manually re-fetched the registration, the result of that has precedence!
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
    Effect::new(move |_| {
        registration.with(|reg| match reg {
            Ok(registration) => {
                // We must be informed when an updated SW is available.
                registration.set_onupdatefound(Some(fetch_registration.as_ref().unchecked_ref()));

                // Trigger a check to see IF an updated SW is available.
                update_sw.dispatch(registration.clone());

                // If a SW is installing, we must be notified if its state changes!
                if let Some(sw) = registration.installing() {
                    sw.set_onstatechange(Some(fetch_registration.as_ref().unchecked_ref()));
                }
            }
            Err(err) => match err {
                ServiceWorkerRegistrationError::Js(err) => {
                    warn!("ServiceWorker registration failed: {err:?}")
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
        check_for_update: move || {
            registration.with(|reg| {
                if let Ok(reg) = reg {
                    update_sw.dispatch(reg.clone());
                }
            })
        },
        skip_waiting: move || {
            registration.with_untracked(|reg| if let Ok(reg) = reg {
                match reg.waiting() {
                    Some(sw) => {
                        debug_warn!("Updating to newly installed SW...");
                        if let Err(err) = sw.post_message(&JsValue::from_str(&options.skip_waiting_message)) {
                            warn!("Could not send message to active SW: Error: {err:?}");
                        }
                    },
                    None => {
                        warn!("You tried to update the SW while no new SW was waiting. This is probably a bug.");
                    },
                }
            });
        },
    }
}

/// Options for [`use_service_worker_with_options`].
#[derive(DefaultBuilder)]
pub struct UseServiceWorkerOptions {
    /// The name of your service-worker file. Must be deployed alongside your app.
    /// The default name is 'service-worker.js'.
    #[builder(into)]
    script_url: String,

    /// The message sent to a waiting ServiceWorker when you call the `skip_waiting` callback.
    /// The callback is part of the return type of [`use_service_worker`]!
    /// The default message is 'skipWaiting'.
    #[builder(into)]
    skip_waiting_message: String,

    /// What should happen when a new service worker was activated?
    /// The default implementation reloads the current page.
    on_controller_change: Arc<dyn Fn()>,
}

impl Default for UseServiceWorkerOptions {
    fn default() -> Self {
        Self {
            script_url: "service-worker.js".into(),
            skip_waiting_message: "skipWaiting".into(),
            on_controller_change: Arc::new(move || {
                use std::ops::Deref;
                if let Some(window) = use_window().deref() {
                    if let Err(err) = window.location().reload() {
                        warn!(
                            "Detected a ServiceWorkerController change but the page reload failed! Error: {err:?}"
                        );
                    }
                }
            }),
        }
    }
}

/// Return type of [`use_service_worker`].
pub struct UseServiceWorkerReturn<CheckFn, SkipFn>
where
    CheckFn: Fn() + Clone,
    SkipFn: Fn() + Clone,
{
    /// The current registration state.
    pub registration:
        Signal<Result<SendWrapper<ServiceWorkerRegistration>, ServiceWorkerRegistrationError>>,

    /// Whether a SW is currently installing.
    pub installing: Signal<bool>,

    /// Whether a SW was installed and is now awaiting activation.
    pub waiting: Signal<bool>,

    /// Whether a SW is active.
    pub active: Signal<bool>,

    /// Check for a ServiceWorker update.
    pub check_for_update: CheckFn,

    /// Call this to activate a new ("waiting") SW if one is available.
    /// Calling this while the [`UseServiceWorkerReturn::waiting`] signal resolves to false has no effect.
    pub skip_waiting: SkipFn,
}

struct ServiceWorkerScriptUrl(pub String);

#[derive(Debug, Clone)]
pub enum ServiceWorkerRegistrationError {
    Js(SendWrapper<JsValue>),
    NeverQueried,
}

/// A leptos action which asynchronously checks for ServiceWorker updates, given an existing ServiceWorkerRegistration.
fn create_action_update() -> Action<
    SendWrapper<ServiceWorkerRegistration>,
    Result<SendWrapper<ServiceWorkerRegistration>, SendWrapper<JsValue>>,
> {
    Action::new_unsync(
        move |registration: &SendWrapper<ServiceWorkerRegistration>| {
            let registration = registration.clone();
            async move {
                match registration.update() {
                    Ok(promise) => js_fut!(promise)
                        .await
                        .and_then(|ok| ok.dyn_into::<ServiceWorkerRegistration>())
                        .map(SendWrapper::new)
                        .map_err(SendWrapper::new),
                    Err(err) => Err(SendWrapper::new(err)),
                }
            }
        },
    )
}

/// A leptos action which asynchronously creates or updates and than retrieves the ServiceWorkerRegistration.
fn create_action_create_or_update_registration() -> Action<
    ServiceWorkerScriptUrl,
    Result<SendWrapper<ServiceWorkerRegistration>, SendWrapper<JsValue>>,
> {
    Action::new_unsync(move |script_url: &ServiceWorkerScriptUrl| {
        let script_url = script_url.0.to_owned();
        async move {
            if let Some(navigator) = use_window().navigator() {
                js_fut!(navigator.service_worker().register(script_url.as_str()))
                    .await
                    .and_then(|ok| ok.dyn_into::<ServiceWorkerRegistration>())
                    .map(SendWrapper::new)
                    .map_err(SendWrapper::new)
            } else {
                Err(SendWrapper::new(JsValue::from_str("no navigator")))
            }
        }
    })
}

/// A leptos action which asynchronously fetches the current ServiceWorkerRegistration.
fn create_action_get_registration(
) -> Action<(), Result<SendWrapper<ServiceWorkerRegistration>, SendWrapper<JsValue>>> {
    Action::new_unsync(move |(): &()| async move {
        if let Some(navigator) = use_window().navigator() {
            js_fut!(navigator.service_worker().get_registration())
                .await
                .and_then(|ok| ok.dyn_into::<ServiceWorkerRegistration>())
                .map(SendWrapper::new)
                .map_err(SendWrapper::new)
        } else {
            Err(SendWrapper::new(JsValue::from_str("no navigator")))
        }
    })
}
