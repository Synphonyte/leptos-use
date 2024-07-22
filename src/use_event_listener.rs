use crate::core::ElementMaybeSignal;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::ev::EventDescriptor;

cfg_if! { if #[cfg(not(feature = "ssr"))] {
    use crate::{watch_with_options, WatchOptions};
    use leptos::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
}}

/// Use EventListener with ease.
/// Register using [addEventListener](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener) on mounted,
/// and [removeEventListener](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener) automatically on cleanup.
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::ev::visibilitychange;
/// # use leptos::logging::log;
/// # use leptos_use::{use_document, use_event_listener};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// use_event_listener(use_document(), visibilitychange, |evt| {
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
/// # use leptos::prelude::*;
/// # use leptos::ev::click;
/// # use leptos::logging::log;
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
/// let (cond, set_cond) = signal(true);
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
/// # use leptos::prelude::*;
/// # use leptos::ev::keydown;
/// # use leptos::logging::log;
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
/// On the server this amounts to a noop.
pub fn use_event_listener<Ev, El, T, F>(target: El, event: Ev, handler: F) -> impl Fn() + Clone
where
    Ev: EventDescriptor + 'static,
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
    F: FnMut(<Ev as EventDescriptor>::EventType) + 'static,
{
    use_event_listener_with_options(target, event, handler, UseEventListenerOptions::default())
}

/// Version of [`use_event_listener`] that takes `web_sys::AddEventListenerOptions`. See the docs for [`use_event_listener`] for how to use.
#[cfg_attr(feature = "ssr", allow(unused_variables))]
#[allow(unused_mut)]
pub fn use_event_listener_with_options<Ev, El, T, F>(
    target: El,
    event: Ev,
    mut handler: F,
    options: UseEventListenerOptions,
) -> impl Fn() + Clone
where
    Ev: EventDescriptor + 'static,
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
    F: FnMut(<Ev as EventDescriptor>::EventType) + 'static,
{
    #[cfg(feature = "ssr")]
    {
        || {}
    }

    #[cfg(not(feature = "ssr"))]
    {
        use leptos::prelude::diagnostics::SpecialNonReactiveZone;
        use send_wrapper::SendWrapper;
        let event_name = event.name();
        let closure_js = Closure::wrap(Box::new(move |e| {
            #[cfg(debug_assertions)]
            let _z = SpecialNonReactiveZone::enter();

            handler(e);
        }) as Box<dyn FnMut(_)>)
        .into_js_value();

        let cleanup_fn = {
            let closure_js = closure_js.clone();
            let options = options.as_add_event_listener_options();

            move |element: &web_sys::EventTarget| {
                let _ = element.remove_event_listener_with_callback_and_event_listener_options(
                    &event_name,
                    closure_js.as_ref().unchecked_ref(),
                    options.unchecked_ref(),
                );
            }
        };

        let event_name = event.name();

        let signal = target.into();

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
                        let options = options.as_add_event_listener_options();

                        _ = element
                            .add_event_listener_with_callback_and_add_event_listener_options(
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

        let cleanup_stop = SendWrapper::new(stop.clone());
        on_cleanup(move || cleanup_stop());

        stop
    }
}

/// Options for [`use_event_listener_with_options`].
#[derive(DefaultBuilder, Default, Copy, Clone)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct UseEventListenerOptions {
    /// A boolean value indicating that events of this type will be dispatched to
    /// the registered `listener` before being dispatched to any `EventTarget`
    /// beneath it in the DOM tree. If not specified, defaults to `false`.
    capture: bool,

    /// A boolean value indicating that the `listener` should be invoked at most
    /// once after being added. If `true`, the `listener` would be automatically
    /// removed when invoked. If not specified, defaults to `false`.
    once: bool,

    /// A boolean value that, if `true`, indicates that the function specified by
    /// `listener` will never call
    /// [`preventDefault()`](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault "preventDefault()").
    /// If a passive listener does call `preventDefault()`, the user agent will do
    /// nothing other than generate a console warning. If not specified,
    /// defaults to `false` – except that in browsers other than Safari,
    /// defaults to `true` for the
    /// [`wheel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event "wheel"),
    /// [`mousewheel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousewheel_event "mousewheel"),
    /// [`touchstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchstart_event "touchstart") and
    /// [`touchmove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchmove_event "touchmove")
    /// events. See [Improving scrolling performance with passive listeners](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#improving_scrolling_performance_with_passive_listeners)
    /// to learn more.
    #[builder(into)]
    passive: Option<bool>,
}

impl UseEventListenerOptions {
    #[cfg_attr(feature = "ssr", allow(dead_code))]
    fn as_add_event_listener_options(&self) -> web_sys::AddEventListenerOptions {
        let UseEventListenerOptions {
            capture,
            once,
            passive,
        } = self;

        let mut options = web_sys::AddEventListenerOptions::new();
        options.capture(*capture);
        options.once(*once);
        if let Some(passive) = passive {
            options.passive(*passive);
        }

        options
    }
}
