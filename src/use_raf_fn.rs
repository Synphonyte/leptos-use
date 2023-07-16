use crate::utils::Pausable;
use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

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
/// # use leptos::*;
/// # use leptos_use::use_raf_fn;
/// use leptos_use::utils::Pausable;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let (count, set_count) = create_signal(cx, 0);
///
/// let Pausable { pause, resume, is_active } = use_raf_fn(cx, move |_| {
///     set_count.update(|count| *count += 1);
/// });
///
/// view! { cx, <div>Count: { count }</div> }
/// }
/// ```
///
/// You can use `use_raf_fn_with_options` and set `immediate` to `false`. In that case
/// you have to call `resume()` before the `callback` is executed.
pub fn use_raf_fn(
    cx: Scope,
    callback: impl Fn(UseRafFnCallbackArgs) + Clone + 'static,
) -> Pausable<impl Fn() + Clone, impl Fn() + Clone> {
    use_raf_fn_with_options(cx, callback, UseRafFnOptions::default())
}

/// Version of [`use_raf_fn`] that takes a `UseRafFnOptions`. See [`use_raf_fn`] for how to use.
pub fn use_raf_fn_with_options(
    cx: Scope,
    callback: impl Fn(UseRafFnCallbackArgs) + Clone + 'static,
    options: UseRafFnOptions,
) -> Pausable<impl Fn() + Clone, impl Fn() + Clone> {
    let UseRafFnOptions { immediate } = options;

    let previous_frame_timestamp = store_value(cx, 0.0_f64);
    let raf_handle = store_value(cx, None::<i32>);

    let (is_active, set_active) = create_signal(cx, false);

    let loop_ref = Rc::new(RefCell::new(Box::new(|_: f64| {}) as Box<dyn Fn(f64)>));

    let request_next_frame = {
        let loop_ref = Rc::clone(&loop_ref);

        move || {
            let loop_ref = Rc::clone(&loop_ref);

            raf_handle.set_value(
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
    };

    let loop_fn = {
        let request_next_frame = request_next_frame.clone();

        move |timestamp: f64| {
            if !is_active.get() {
                return;
            }

            let prev_timestamp = previous_frame_timestamp.get_value();
            let delta = if prev_timestamp > 0.0 {
                timestamp - prev_timestamp
            } else {
                0.0
            };

            callback(UseRafFnCallbackArgs { delta, timestamp });

            previous_frame_timestamp.set_value(timestamp);

            request_next_frame();
        }
    };

    let _ = loop_ref.replace(Box::new(loop_fn));

    let resume = move || {
        if !is_active.get() {
            set_active.set(true);
            request_next_frame();
        }
    };

    let pause = move || {
        set_active.set(false);

        let handle = raf_handle.get_value();
        if let Some(handle) = handle {
            let _ = window().cancel_animation_frame(handle);
        }
        raf_handle.set_value(None);
    };

    if immediate {
        resume();
    }

    on_cleanup(cx, pause);

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

    /// Time elapsed since the creation of the web page. See [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/API/DOMHighResTimeStamp#the_time_origin Time origin).
    pub timestamp: f64,
}
