use crate::core::{Direction, Directions, IntoElementMaybeSignal};
use crate::UseEventListenerOptions;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::rc::Rc;

cfg_if! { if #[cfg(not(feature = "ssr"))] {
use crate::use_event_listener::use_event_listener_with_options;
use crate::{
    sendwrap_fn, use_debounce_fn_with_arg, use_throttle_fn_with_arg_and_options, ThrottleOptions,
};
use leptos::ev;
use leptos::ev::scrollend;
use wasm_bindgen::JsCast;

/// We have to check if the scroll amount is close enough to some threshold in order to
/// more accurately calculate arrivedState. This is because scrollTop/scrollLeft are non-rounded
/// numbers, while scrollHeight/scrollWidth and clientHeight/clientWidth are rounded.
/// https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollHeight#determine_if_an_element_has_been_totally_scrolled
const ARRIVED_STATE_THRESHOLD_PIXELS: f64 = 1.0;
}}

/// Reactive scroll position and state.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_scroll)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::ev::resize;
/// # use leptos::html::Div;
/// # use leptos_use::{use_scroll, UseScrollReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let element = NodeRef::<Div>::new();
///
/// let UseScrollReturn {
///     x, y, set_x, set_y, is_scrolling, arrived_state, directions, ..
/// } = use_scroll(element);
///
/// view! {
///     <div node_ref=element>"..."</div>
/// }
/// # }
/// ```
///
/// ### With Offsets
///
/// You can provide offsets when you use [`use_scroll_with_options`].
/// These offsets are thresholds in pixels when a side is considered to have arrived. This is reflected in the return field `arrived_state`.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Div;
/// # use leptos::ev::resize;
/// # use leptos_use::{use_scroll_with_options, UseScrollReturn, UseScrollOptions, ScrollOffset};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// # let element = NodeRef::<Div>::new();
/// #
/// let UseScrollReturn {
///     x,
///     y,
///     set_x,
///     set_y,
///     is_scrolling,
///     arrived_state,
///     directions,
///     ..
/// } = use_scroll_with_options(
///     element,
///     UseScrollOptions::default().offset(ScrollOffset {
///         top: 30.0,
///         bottom: 30.0,
///         right: 30.0,
///         left: 30.0,
///     }),
/// );
/// #
/// #     view! { /// #         <div node_ref=element>"..."</div>
/// #     }
/// # }
/// ```
///
/// ### Setting Scroll Position
///
/// Set the `x` and `y` values to make the element scroll to that position.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Div;
/// # use leptos::ev::resize;
/// # use leptos_use::{use_scroll, UseScrollReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let element = NodeRef::<Div>::new();
///
/// let UseScrollReturn {
///     x, y, set_x, set_y, ..
/// } = use_scroll(element);
///
/// view! {
///     <div node_ref=element>"..."</div>
///     <button on:click=move |_| set_x(x.get_untracked() + 10.0)>"Scroll right 10px"</button>
///     <button on:click=move |_| set_y(y.get_untracked() + 10.0)>"Scroll down 10px"</button>
/// }
/// # }
/// ```
///
/// ### Smooth Scrolling
///
/// Set `behavior: smooth` to enable smooth scrolling. The `behavior` option defaults to `auto`,
/// which means no smooth scrolling. See the `behavior` option on
/// [Element.scrollTo](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo) for more information.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::ev::resize;
/// # use leptos::html::Div;
/// # use leptos_use::{use_scroll_with_options, UseScrollReturn, UseScrollOptions, ScrollBehavior};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// # let element = NodeRef::<Div>::new();
/// #
/// let UseScrollReturn {
///     x, y, set_x, set_y, ..
/// } = use_scroll_with_options(
///     element,
///     UseScrollOptions::default().behavior(ScrollBehavior::Smooth),
/// );
/// #
/// # view! { /// #     <div node_ref=element>"..."</div>
/// # }
/// # }
/// ```
///
/// or as a `Signal`:
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::ev::resize;
/// # use leptos::html::Div;
/// # use leptos_use::{use_scroll_with_options, UseScrollReturn, UseScrollOptions, ScrollBehavior};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// # let element = NodeRef::<Div>::new();
/// #
/// let (smooth, set_smooth) = signal(false);
///
/// let behavior = Signal::derive(move || {
///     if smooth.get() { ScrollBehavior::Smooth } else { ScrollBehavior::Auto }
/// });
///
/// let UseScrollReturn {
///     x, y, set_x, set_y, ..
/// } = use_scroll_with_options(
///     element,
///     UseScrollOptions::default().behavior(behavior),
/// );
/// #
/// # view! { /// #     <div node_ref=element>"..."</div>
/// # }
/// # }
/// ```
///
/// ## SendWrapped Return
///
/// The returned closures `set_x`, `set_y` and `measure` are sendwrapped functions. They can
/// only be called from the same thread that called `use_scroll`.
///
/// ## Server-Side Rendering
///
/// On the server this returns signals that don't change and setters that are noops.
pub fn use_scroll<El, M>(
    element: El,
) -> UseScrollReturn<
    impl Fn(f64) + Clone + Send + Sync,
    impl Fn(f64) + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
{
    use_scroll_with_options(element, Default::default())
}

/// Version of [`use_scroll`] with options. See [`use_scroll`] for how to use.
#[cfg_attr(feature = "ssr", allow(unused_variables))]
pub fn use_scroll_with_options<El, M>(
    element: El,
    options: UseScrollOptions,
) -> UseScrollReturn<
    impl Fn(f64) + Clone + Send + Sync,
    impl Fn(f64) + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
{
    let (internal_x, set_internal_x) = signal(0.0);
    let (internal_y, set_internal_y) = signal(0.0);

    let (is_scrolling, set_is_scrolling) = signal(false);

    let arrived_state = RwSignal::new(Directions {
        left: true,
        right: false,
        top: true,
        bottom: false,
    });
    let directions = RwSignal::new(Directions {
        left: false,
        right: false,
        top: false,
        bottom: false,
    });

    let set_x;
    let set_y;
    let measure;

    #[cfg(feature = "ssr")]
    {
        set_x = |_| {};
        set_y = |_| {};
        measure = || {};
    }

    #[cfg(not(feature = "ssr"))]
    {
        let signal = element.into_element_maybe_signal();
        let behavior = options.behavior;

        let scroll_to = move |x: Option<f64>, y: Option<f64>| {
            let element = signal.get_untracked();

            if let Some(element) = element {
                let scroll_options = web_sys::ScrollToOptions::new();
                scroll_options.set_behavior(behavior.get_untracked().into());

                if let Some(x) = x {
                    scroll_options.set_left(x);
                }
                if let Some(y) = y {
                    scroll_options.set_top(y);
                }

                element.scroll_to_with_scroll_to_options(&scroll_options);
            }
        };

        set_x = sendwrap_fn!(move |x| scroll_to(Some(x), None));

        set_y = sendwrap_fn!(move |y| scroll_to(None, Some(y)));

        let on_scroll_end = {
            let on_stop = Rc::clone(&options.on_stop);

            move |e| {
                if !is_scrolling.try_get_untracked().unwrap_or_default() {
                    return;
                }

                set_is_scrolling.set(false);
                directions.update(|directions| {
                    directions.left = false;
                    directions.right = false;
                    directions.top = false;
                    directions.bottom = false;
                    on_stop.clone()(e);
                });
            }
        };

        let throttle = options.throttle;

        let on_scroll_end_debounced =
            use_debounce_fn_with_arg(on_scroll_end.clone(), throttle + options.idle);

        let offset = options.offset;

        let set_arrived_state = move |target: web_sys::Element| {
            let style = window()
                .get_computed_style(&target)
                .expect("failed to get computed style");

            if let Some(style) = style {
                let display = style
                    .get_property_value("display")
                    .expect("failed to get display");
                let flex_direction = style
                    .get_property_value("flex-direction")
                    .expect("failed to get flex-direction");

                let scroll_left = target.scroll_left() as f64;
                let scroll_left_abs = scroll_left.abs();

                directions.update(|directions| {
                    directions.left = scroll_left < internal_x.get_untracked();
                    directions.right = scroll_left > internal_x.get_untracked();
                });

                let left = scroll_left_abs <= offset.left;
                let right = scroll_left_abs + target.client_width() as f64
                    >= target.scroll_width() as f64 - offset.right - ARRIVED_STATE_THRESHOLD_PIXELS;

                arrived_state.update(|arrived_state| {
                    if display == "flex" && flex_direction == "row-reverse" {
                        arrived_state.left = right;
                        arrived_state.right = left;
                    } else {
                        arrived_state.left = left;
                        arrived_state.right = right;
                    }
                });
                set_internal_x.set(scroll_left);

                let mut scroll_top = target.scroll_top() as f64;

                // patch for mobile compatibility
                if target == document().unchecked_into::<web_sys::Element>() && scroll_top == 0.0 {
                    scroll_top = document().body().expect("failed to get body").scroll_top() as f64;
                }

                let scroll_top_abs = scroll_top.abs();

                directions.update(|directions| {
                    directions.top = scroll_top < internal_y.get_untracked();
                    directions.bottom = scroll_top > internal_y.get_untracked();
                });

                let top = scroll_top_abs <= offset.top;
                let bottom = scroll_top_abs + target.client_height() as f64
                    >= target.scroll_height() as f64
                        - offset.bottom
                        - ARRIVED_STATE_THRESHOLD_PIXELS;

                // reverse columns and rows behave exactly the other way around,
                // bottom is treated as top and top is treated as the negative version of bottom
                arrived_state.update(|arrived_state| {
                    if display == "flex" && flex_direction == "column-reverse" {
                        arrived_state.top = bottom;
                        arrived_state.bottom = top;
                    } else {
                        arrived_state.top = top;
                        arrived_state.bottom = bottom;
                    }
                });

                set_internal_y.set(scroll_top);
            }
        };

        let on_scroll_handler = {
            let on_scroll = Rc::clone(&options.on_scroll);

            move |e: web_sys::Event| {
                let target: web_sys::Element = event_target(&e);

                set_arrived_state(target);
                set_is_scrolling.set(true);

                on_scroll_end_debounced.clone()(e.clone());
                on_scroll.clone()(e);
            }
        };

        let target = Signal::derive_local(move || {
            let element = signal.get();
            element.map(|element| element.unchecked_into::<web_sys::EventTarget>())
        });

        if throttle >= 0.0 {
            let throttled_scroll_handler = use_throttle_fn_with_arg_and_options(
                on_scroll_handler.clone(),
                throttle,
                ThrottleOptions {
                    trailing: true,
                    leading: false,
                },
            );

            let handler = move |e: web_sys::Event| {
                throttled_scroll_handler.clone()(e);
            };

            let _ = use_event_listener_with_options::<
                _,
                Signal<Option<web_sys::EventTarget>, LocalStorage>,
                _,
                _,
            >(target, ev::scroll, handler, options.event_listener_options);
        } else {
            let _ = use_event_listener_with_options::<
                _,
                Signal<Option<web_sys::EventTarget>, LocalStorage>,
                _,
                _,
            >(
                target,
                ev::scroll,
                on_scroll_handler,
                options.event_listener_options,
            );
        }

        let _ = use_event_listener_with_options::<
            _,
            Signal<Option<web_sys::EventTarget>, LocalStorage>,
            _,
            _,
        >(
            target,
            scrollend,
            on_scroll_end,
            options.event_listener_options,
        );

        measure = sendwrap_fn!(move || {
            if let Some(el) = signal.try_get_untracked().flatten() {
                set_arrived_state(el);
            }
        });
    }

    UseScrollReturn {
        x: internal_x.into(),
        set_x,
        y: internal_y.into(),
        set_y,
        is_scrolling: is_scrolling.into(),
        arrived_state: arrived_state.into(),
        directions: directions.into(),
        measure,
    }
}

/// Options for [`use_scroll`].
#[derive(DefaultBuilder)]
/// Options for [`use_scroll_with_options`].
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct UseScrollOptions {
    /// Throttle time in milliseconds for the scroll events. Defaults to 0 (disabled).
    throttle: f64,

    /// After scrolling ends we wait idle + throttle milliseconds before we consider scrolling to have stopped.
    /// Defaults to 200.
    idle: f64,

    /// Threshold in pixels when we consider a side to have arrived (`UseScrollReturn::arrived_state`).
    offset: ScrollOffset,

    /// Callback when scrolling is happening.
    on_scroll: Rc<dyn Fn(web_sys::Event)>,

    /// Callback when scrolling stops (after `idle` + `throttle` milliseconds have passed).
    on_stop: Rc<dyn Fn(web_sys::Event)>,

    /// Options passed to the `addEventListener("scroll", ...)` call
    event_listener_options: UseEventListenerOptions,

    /// When changing the `x` or `y` signals this specifies the scroll behaviour.
    /// Can be `Auto` (= not smooth) or `Smooth`. Defaults to `Auto`.
    #[builder(into)]
    behavior: Signal<ScrollBehavior>,
}

impl Default for UseScrollOptions {
    fn default() -> Self {
        Self {
            throttle: 0.0,
            idle: 200.0,
            offset: ScrollOffset::default(),
            on_scroll: Rc::new(|_| {}),
            on_stop: Rc::new(|_| {}),
            event_listener_options: Default::default(),
            behavior: Default::default(),
        }
    }
}

/// The scroll behavior.
/// Can be `Auto` (= not smooth) or `Smooth`. Defaults to `Auto`.
#[derive(Default, Copy, Clone)]
pub enum ScrollBehavior {
    #[default]
    Auto,
    Smooth,
}

impl From<ScrollBehavior> for web_sys::ScrollBehavior {
    fn from(val: ScrollBehavior) -> Self {
        match val {
            ScrollBehavior::Auto => web_sys::ScrollBehavior::Auto,
            ScrollBehavior::Smooth => web_sys::ScrollBehavior::Smooth,
        }
    }
}

/// The return value of [`use_scroll`].
pub struct UseScrollReturn<SetXFn, SetYFn, MFn>
where
    SetXFn: Fn(f64) + Clone + Send + Sync,
    SetYFn: Fn(f64) + Clone + Send + Sync,
    MFn: Fn() + Clone + Send + Sync,
{
    /// X coordinate of scroll position
    pub x: Signal<f64>,

    /// Sets the value of `x`. This does also scroll the element.
    pub set_x: SetXFn,

    /// Y coordinate of scroll position
    pub y: Signal<f64>,

    /// Sets the value of `y`. This does also scroll the element.
    pub set_y: SetYFn,

    /// Is true while the element is being scrolled.
    pub is_scrolling: Signal<bool>,

    /// Sets the field that represents a direction to true if the
    /// element is scrolled all the way to that side.
    pub arrived_state: Signal<Directions>,

    /// The directions in which the element is being scrolled are set to true.
    pub directions: Signal<Directions>,

    /// Re-evaluates the `arrived_state`.
    pub measure: MFn,
}

#[derive(Default, Copy, Clone, Debug)]
/// Threshold in pixels when we consider a side to have arrived (`UseScrollReturn::arrived_state`).
pub struct ScrollOffset {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl ScrollOffset {
    /// Sets the value of the provided direction
    pub fn set_direction(mut self, direction: Direction, value: f64) -> Self {
        match direction {
            Direction::Top => self.top = value,
            Direction::Bottom => self.bottom = value,
            Direction::Left => self.left = value,
            Direction::Right => self.right = value,
        }

        self
    }
}
