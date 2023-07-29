use crate::core::ElementMaybeSignal;
use crate::{watch_with_options, WatchOptions};
use leptos::ev::EventDescriptor;
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// Use EventListener with ease.
/// Register using [addEventListener](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener) on mounted,
/// and [removeEventListener](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener) automatically on cleanup.
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos::ev::visibilitychange;
/// # use leptos_use::use_event_listener;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// use_event_listener(document(), visibilitychange, |evt| {
///     log!("{:?}", evt);
/// });
/// #    view! { }
/// # }
/// ```
///
/// You can also pass a [`NodeRef`](https://docs.rs/leptos/latest/leptos/struct.NodeRef.html) as the event target, [`use_event_listener`] will unregister the previous event and register
/// the new one when you change the target.
///
/// ```
/// # use leptos::*;
/// # use leptos::ev::click;
/// # use leptos_use::use_event_listener;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let element = create_node_ref();
///
/// use_event_listener(element, click, |evt| {
///     log!("click from element {:?}", event_target::<web_sys::HtmlDivElement>(&evt));
/// });
///
/// let (cond, set_cond) = create_signal(true);
///
/// view! {
///     <Show
///         when=move || cond.get()
///         fallback=move || view! { <div node_ref=element>"Condition false"</div> }
///     >
///         <div node_ref=element>"Condition true"</div>
///     </Show>
/// }
/// # }
/// ```
///
/// You can also call the returned to unregister the listener.
///
/// ```
/// # use leptos::*;
/// # use leptos::ev::keydown;
/// # use web_sys::KeyboardEvent;
/// # use leptos_use::use_event_listener;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let cleanup = use_event_listener(document().body(), keydown, |evt: KeyboardEvent| {
///     log!("{}", &evt.key());
/// });
///
/// cleanup();
/// #
/// #    view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// Please refer to ["Functions with Target Elements"](https://leptos-use.rs/server_side_rendering.html#functions-with-target-elements)
pub fn use_event_listener<Ev, El, T, F>(target: El, event: Ev, handler: F) -> impl Fn() + Clone
where
    Ev: EventDescriptor + 'static,
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
    F: FnMut(<Ev as EventDescriptor>::EventType) + 'static,
{
    use_event_listener_with_options(
        target,
        event,
        handler,
        web_sys::AddEventListenerOptions::new(),
    )
}

/// Version of [`use_event_listener`] that takes `web_sys::AddEventListenerOptions`. See the docs for [`use_event_listener`] for how to use.
pub fn use_event_listener_with_options<Ev, El, T, F>(
    target: El,
    event: Ev,
    handler: F,
    options: web_sys::AddEventListenerOptions,
) -> impl Fn() + Clone
where
    Ev: EventDescriptor + 'static,
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
    F: FnMut(<Ev as EventDescriptor>::EventType) + 'static,
{
    let event_name = event.name();
    let closure_js = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>).into_js_value();

    let cleanup_fn = {
        let closure_js = closure_js.clone();
        let options = options.clone();

        move |element: &web_sys::EventTarget| {
            let _ = element.remove_event_listener_with_callback_and_event_listener_options(
                &event_name,
                closure_js.as_ref().unchecked_ref(),
                options.unchecked_ref(),
            );
        }
    };

    let event_name = event.name();

    let signal = (target).into();

    let prev_element = Rc::new(RefCell::new(None::<web_sys::EventTarget>));

    let cleanup_prev_element = {
        let prev_element = prev_element.clone();

        move || {
            if let Some(element) = prev_element.take() {
                cleanup_fn(&element);
            }
        }
    };

    let stop_watch = {
        let cleanup_prev_element = cleanup_prev_element.clone();

        watch_with_options(
            move || signal.get().map(|e| e.into()),
            move |element, _, _| {
                cleanup_prev_element();
                prev_element.replace(element.clone());

                if let Some(element) = element {
                    _ = element.add_event_listener_with_callback_and_add_event_listener_options(
                        &event_name,
                        closure_js.as_ref().unchecked_ref(),
                        &options,
                    );
                }
            },
            WatchOptions::default().immediate(true),
        )
    };

    let stop = move || {
        stop_watch();
        cleanup_prev_element();
    };

    on_cleanup(stop.clone());

    stop
}
