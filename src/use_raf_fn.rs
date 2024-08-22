use crate::utils::Pausable;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Call function on every requestAnimationFrame.
/// With controls of pausing and resuming.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_raf_fn)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_raf_fn;
/// use leptos_use::utils::Pausable;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (count, set_count) = signal(0);
///
/// let Pausable { pause, resume, is_active } = use_raf_fn(move |_| {
///     set_count.update(|count| *count += 1);
/// });
///
/// view! { <div>Count: { count }</div> }
/// }
/// ```
///
/// You can use `use_raf_fn_with_options` and set `immediate` to `false`. In that case
/// you have to call `resume()` before the `callback` is executed.
///
/// ## Server-Side Rendering
///
/// On the server this does basically nothing. The provided closure will never be called.
pub fn use_raf_fn(
    callback: impl Fn(UseRafFnCallbackArgs) + 'static,
) -> Pausable<impl Fn() + Clone, impl Fn() + Clone> {
    use_raf_fn_with_options(callback, UseRafFnOptions::default())
}

/// Version of [`use_raf_fn`] that takes a `UseRafFnOptions`. See [`use_raf_fn`] for how to use.
pub fn use_raf_fn_with_options(
    callback: impl Fn(UseRafFnCallbackArgs) + 'static,
    options: UseRafFnOptions,
) -> Pausable<impl Fn() + Clone, impl Fn() + Clone> {
    let UseRafFnOptions { immediate } = options;

    let raf_handle = Rc::new(Cell::new(None::<i32>));

    let (is_active, set_active) = signal(false);

    let loop_ref = Rc::new(RefCell::new(Box::new(|_: f64| {}) as Box<dyn Fn(f64)>));

    let request_next_frame = {
        cfg_if! { if #[cfg(feature = "ssr")] {
            move || ()
        } else {
            use wasm_bindgen::JsCast;
            use wasm_bindgen::closure::Closure;

            let loop_ref = Rc::clone(&loop_ref);
            let raf_handle = Rc::clone(&raf_handle);

            move || {
                let loop_ref = Rc::clone(&loop_ref);

                raf_handle.set(
                    window()
                        .request_animation_frame(
                            Closure::once_into_js(move |timestamp: f64| {
                                loop_ref.borrow()(timestamp);
                            })
                            .as_ref()
                            .unchecked_ref(),
                        )
                        .ok(),
                );
            }
        }}
    };

    let loop_fn = {
        #[allow(clippy::clone_on_copy)]
        let request_next_frame = request_next_frame.clone();
        let previous_frame_timestamp = Cell::new(0.0_f64);

        move |timestamp: f64| {
            if !is_active.try_get_untracked().unwrap_or_default() {
                return;
            }

            let prev_timestamp = previous_frame_timestamp.get();
            let delta = if prev_timestamp > 0.0 {
                timestamp - prev_timestamp
            } else {
                0.0
            };

            #[cfg(debug_assertions)]
            let zone = leptos::prelude::diagnostics::SpecialNonReactiveZone::enter();

            callback(UseRafFnCallbackArgs { delta, timestamp });

            #[cfg(debug_assertions)]
            drop(zone);

            previous_frame_timestamp.set(timestamp);

            request_next_frame();
        }
    };

    let _ = loop_ref.replace(Box::new(loop_fn));

    let resume = move || {
        if !is_active.get_untracked() {
            set_active.set(true);
            request_next_frame();
        }
    };

    let pause = move || {
        set_active.set(false);

        let handle = raf_handle.get();
        if let Some(handle) = handle {
            let _ = window().cancel_animation_frame(handle);
        }
        raf_handle.set(None);
    };

    if immediate {
        resume();
    }

    on_cleanup({
        let pause = send_wrapper::SendWrapper::new(pause.clone());
        #[allow(clippy::redundant_closure)]
        move || pause()
    });

    Pausable {
        resume,
        pause,
        is_active: is_active.into(),
    }
}

/// Options for [`use_raf_fn_with_options`].
#[derive(DefaultBuilder)]
pub struct UseRafFnOptions {
    /// Start the requestAnimationFrame loop immediately on creation. Defaults to `true`.
    /// If false the loop will only start when you call `resume()`.
    immediate: bool,
}

impl Default for UseRafFnOptions {
    fn default() -> Self {
        Self { immediate: true }
    }
}

/// Type of the argument for the callback of [`use_raf_fn`].
pub struct UseRafFnCallbackArgs {
    /// Time elapsed between this and the last frame.
    pub delta: f64,

    /// Time elapsed since the creation of the web page. See [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/API/DOMHighResTimeStamp#the_time_origin) Time origin.
    pub timestamp: f64,
}
