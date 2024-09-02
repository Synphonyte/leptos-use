use default_struct_builder::DefaultBuilder;
use std::future::Future;
use thiserror::Error;
use wasm_bindgen::JsValue;
pub use web_sys::LockMode;

/// Rustified [Web Locks API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Locks_API).   
///
/// The **Web Locks API** allows scripts running in one tab or worker to asynchronously acquire a
/// lock, hold it while work is performed, then release it. While held, no other script executing
/// in the same origin can acquire the same lock, which allows a web app running in multiple tabs or
/// workers to coordinate work and the use of resources.
///
/// > This function requires `--cfg=web_sys_unstable_apis` to be activated as
/// > [described in the wasm-bindgen guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_web_lock)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_web_lock;
/// #
/// async fn my_process(_lock: web_sys::Lock) -> i32 {
///     // do sth
///     42
/// }
///
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// spawn_local(async {
///     let res = use_web_lock("my_lock", my_process).await;
///     assert!(matches!(res, Ok(42)));
/// });
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this returns `Err(UseWebLockError::Server)` and the task is not executed.
// #[doc(cfg(feature = "use_web_lock"))]

pub async fn use_web_lock<C, F, R>(name: &str, callback: C) -> Result<R, UseWebLockError>
where
    C: FnOnce(web_sys::Lock) -> F + 'static,
    F: Future<Output = R>,
    R: 'static,
{
    use_web_lock_with_options(name, callback, UseWebLockOptions::default()).await
}

/// Version of [`fn@crate::use_web_lock`] that takes a `UseWebLockOptions`. See [`fn@crate::use_web_lock`] for how to use.
// #[doc(cfg(feature = "use_web_lock"))]
pub async fn use_web_lock_with_options<C, F, R>(
    name: &str,
    callback: C,
    options: UseWebLockOptions,
) -> Result<R, UseWebLockError>
where
    C: FnOnce(web_sys::Lock) -> F + 'static,
    F: Future<Output = R>,
    R: 'static,
{
    #[cfg(feature = "ssr")]
    {
        let _ = name;
        let _ = callback;
        let _ = options;

        Err(UseWebLockError::Server)
    }

    #[cfg(not(feature = "ssr"))]
    {
        use crate::js_fut;
        use leptos::window;
        use std::sync::{Arc, Mutex};
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::future_to_promise;

        let ret_value = Arc::new(Mutex::new(None));

        let handler = Closure::once(Box::new({
            let ret_value = Arc::clone(&ret_value);

            move |lock| {
                future_to_promise(async move {
                    let ret = callback(lock).await;
                    ret_value.lock().expect("Lock failed").replace(ret);
                    Ok(JsValue::null())
                })
            }
        }) as Box<dyn FnOnce(web_sys::Lock) -> _>)
        .into_js_value();

        let lock_promise = window()
            .navigator()
            .locks()
            .request_with_options_and_callback(
                name,
                &options.to_web_sys(),
                handler.unchecked_ref(),
            );

        js_fut!(lock_promise)
            .await
            .map(move |_| {
                Arc::into_inner(ret_value)
                    .expect("Arc has more than one reference still")
                    .into_inner()
                    .expect("Lock failed")
                    .expect("Return value was None")
            })
            .map_err(UseWebLockError::Failed)
    }
}

#[derive(Error, Debug)]
pub enum UseWebLockError {
    #[error("Lock cannot be acquired on the server")]
    Server,

    #[error("Lock failed")]
    Failed(JsValue),
}

/// Options for [`fn@crate::use_web_lock_with_options`].
// #[doc(cfg(feature = "use_web_lock"))]
#[allow(dead_code)]
#[derive(DefaultBuilder)]
pub struct UseWebLockOptions {
    /// The default mode is `LockMode::Exclusive`, but `LockMode::Shared` can be specified.
    /// There can be only one `Exclusive` holder of a lock, but multiple `Shared` requests can be
    /// granted at the same time. This can be used to implement the
    /// [readers-writer pattern](https://en.wikipedia.org/wiki/Readers%E2%80%93writer_lock).
    mode: LockMode,

    /// If `true`, the lock request will fail if the lock cannot be granted immediately without
    /// waiting. The callback is invoked with `null`. Defaults to `false`.
    if_available: bool,

    /// If `true`, then any held locks with the same name will be released, and the request will
    /// be granted, preempting any queued requests for it. Defaults to `false`.
    steal: bool,
    // TODO : add abort signal (this also requires to create a wrapper for AbortSignal similar to UseWindow)
}

#[cfg(not(feature = "ssr"))]
impl UseWebLockOptions {
    fn to_web_sys(&self) -> web_sys::LockOptions {
        let options = web_sys::LockOptions::new();
        options.set_mode(self.mode);
        options.set_if_available(self.if_available);
        options.set_steal(self.steal);

        options
    }
}

impl Default for UseWebLockOptions {
    fn default() -> Self {
        Self {
            mode: LockMode::Exclusive,
            if_available: false,
            steal: false,
        }
    }
}
