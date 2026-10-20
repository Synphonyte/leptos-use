use crate::UseWebLockOptions;
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
/// # use leptos::prelude::*;
/// # use leptos_use_web_lock::use_web_lock;
/// #
/// async fn my_process(_lock: web_sys::Lock) -> i32 {
///     // do sth
///     42
/// }
///
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// leptos::task::spawn_local(async {
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
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this returns `Err(UseWebLockError::Server)` and the task is not executed.
pub async fn use_web_lock<C, F, R>(name: &str, callback: C) -> Result<R, UseWebLockError>
where
    C: FnOnce(web_sys::Lock) -> F + 'static,
    F: Future<Output = R>,
    R: 'static,
{
    use_web_lock_with_options(name, callback, UseWebLockOptions::default()).await
}

/// Version of [`fn@crate::use_web_lock`] that takes a `UseWebLockOptions`. See [`fn@crate::use_web_lock`] for how to use.
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
        use leptos::prelude::window;
        use std::sync::{Arc, Mutex};
        use wasm_bindgen::JsCast;
        use wasm_bindgen::closure::Closure;
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

        let lock_promise = window().navigator().locks().request_with_options(
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
