use default_struct_builder::DefaultBuilder;

/// Options for [`use_event_listener_with_options`](crate::use_event_listener_with_options).
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
    pub(crate) fn as_add_event_listener_options(&self) -> web_sys::AddEventListenerOptions {
        let UseEventListenerOptions {
            capture,
            once,
            passive,
        } = self;

        let options = web_sys::AddEventListenerOptions::new();
        options.set_capture(*capture);
        options.set_once(*once);
        if let Some(passive) = passive {
            options.set_passive(*passive);
        }

        options
    }
}
