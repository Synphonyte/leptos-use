use leptos::reactive::wrappers::read::Signal;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsValue;
use web_sys::ServiceWorkerRegistration;

/// Return type of [`use_service_worker`](crate::use_service_worker).
pub struct UseServiceWorkerReturn<CheckFn, SkipFn>
where
    CheckFn: Fn() + Clone + Send + Sync,
    SkipFn: Fn() + Clone + Send + Sync,
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

#[derive(Debug, Clone)]
pub enum ServiceWorkerRegistrationError {
    Js(SendWrapper<JsValue>),
    NeverQueried,
}
