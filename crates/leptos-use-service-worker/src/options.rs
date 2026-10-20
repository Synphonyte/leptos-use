use crate::use_window;
use default_struct_builder::DefaultBuilder;
use leptos::logging::warn;
use std::sync::Arc;

/// Options for [`use_service_worker_with_options`](crate::use_service_worker_with_options).
#[derive(DefaultBuilder)]
pub struct UseServiceWorkerOptions {
    /// The name of your service-worker file. Must be deployed alongside your app.
    /// The default name is 'service-worker.js'.
    #[builder(into)]
    pub(crate) script_url: String,

    /// The message sent to a waiting ServiceWorker when you call the `skip_waiting` callback.
    /// The callback is part of the return type of [`use_service_worker`](crate::use_service_worker)!
    /// The default message is 'skipWaiting'.
    #[builder(into)]
    pub(crate) skip_waiting_message: String,

    /// What should happen when a new service worker was activated?
    /// The default implementation reloads the current page.
    pub(crate) on_controller_change: Arc<dyn Fn()>,
}

impl Default for UseServiceWorkerOptions {
    fn default() -> Self {
        Self {
            script_url: "service-worker.js".into(),
            skip_waiting_message: "skipWaiting".into(),
            on_controller_change: Arc::new(move || {
                use std::ops::Deref;
                if let Some(window) = use_window().deref()
                    && let Err(err) = window.location().reload()
                {
                    warn!(
                        "Detected a ServiceWorkerController change but the page reload failed! Error: {err:?}"
                    );
                }
            }),
        }
    }
}
