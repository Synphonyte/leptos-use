use crate::core::{Direction, Directions, ElementMaybeSignal};
use crate::{
    use_element_visibility, use_scroll_with_options, ScrollOffset, UseEventListenerOptions,
    UseScrollOptions, UseScrollReturn,
};
use default_struct_builder::DefaultBuilder;
use futures_util::join;
use gloo_timers::future::sleep;
use leptos::prelude::diagnostics::SpecialNonReactiveZone;
use leptos::prelude::wrappers::read::Signal;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use wasm_bindgen::JsCast;

/// Infinite scrolling of the element.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_infinite_scroll)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// use leptos::html::Div;
/// # use leptos_use::{use_infinite_scroll_with_options, UseInfiniteScrollOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = create_node_ref::<Div>();
///
/// let (data, set_data) = signal(vec![1, 2, 3, 4, 5, 6]);
///
/// let _ = use_infinite_scroll_with_options(
///     el,
///     move |_| async move {
///         let len = data.with(|d| d.len());
///         set_data.update(|data| *data = (1..len+6).collect());
///     },
///     UseInfiniteScrollOptions::default().distance(10.0),
/// );
///
/// view! {
///     <div node_ref=el>
///         <For each=move || data.get() key=|i| *i let:item>{ item }</For>
///     </div>
/// }
/// # }
/// ```
///
/// The returned signal is `true` while new data is being loaded.
pub fn use_infinite_scroll<El, T, LFn, LFut>(el: El, on_load_more: LFn) -> Signal<bool>
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>> + Clone + 'static,
    T: Into<web_sys::Element> + Clone + 'static,
    LFn: Fn(ScrollState) -> LFut + Send + Sync + 'static,
    LFut: Future<Output = ()>,
{
    use_infinite_scroll_with_options(el, on_load_more, UseInfiniteScrollOptions::default())
}

/// Version of [`use_infinite_scroll`] that takes a `UseInfiniteScrollOptions`. See [`use_infinite_scroll`] for how to use.
pub fn use_infinite_scroll_with_options<El, T, LFn, LFut>(
    el: El,
    on_load_more: LFn,
    options: UseInfiniteScrollOptions,
) -> Signal<bool>
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>> + Clone + 'static,
    T: Into<web_sys::Element> + Clone + 'static,
    LFn: Fn(ScrollState) -> LFut + Send + Sync + 'static,
    LFut: Future<Output = ()>,
{
    let UseInfiniteScrollOptions {
        distance,
        direction,
        interval,
        on_scroll,
        event_listener_options,
    } = options;

    let on_load_more = StoredValue::new(on_load_more);

    let UseScrollReturn {
        x,
        y,
        is_scrolling,
        arrived_state,
        directions,
        measure,
        ..
    } = use_scroll_with_options(
        el.clone(),
        UseScrollOptions::default()
            .on_scroll(move |evt| on_scroll(evt))
            .event_listener_options(event_listener_options)
            .offset(ScrollOffset::default().set_direction(direction, distance)),
    );

    let state = ScrollState {
        x,
        y,
        is_scrolling,
        arrived_state,
        directions,
    };

    let (is_loading, set_loading) = signal(false);

    let el = el.into();
    let observed_element = Signal::derive(move || {
        let el = el.get();

        el.map(|el| {
            let el = el.into();

            if el.is_instance_of::<web_sys::Window>() || el.is_instance_of::<web_sys::Document>() {
                SendWrapper::new(
                    document()
                        .document_element()
                        .expect("document element not found"),
                )
            } else {
                SendWrapper::new(el)
            }
        })
    });

    let is_element_visible = use_element_visibility(observed_element);

    let check_and_load = StoredValue::new(None::<Arc<dyn Fn() + Send + Sync>>);

    check_and_load.set_value(Some(Arc::new({
        let measure = measure.clone();

        move || {
            let observed_element = observed_element.get_untracked();

            if !is_element_visible.get_untracked() {
                return;
            }

            if let Some(observed_element) = observed_element {
                let scroll_height = observed_element.scroll_height();
                let client_height = observed_element.client_height();
                let scroll_width = observed_element.scroll_width();
                let client_width = observed_element.client_width();

                let is_narrower = if direction == Direction::Bottom || direction == Direction::Top {
                    scroll_height <= client_height
                } else {
                    scroll_width <= client_width
                };

                if (state.arrived_state.get_untracked().get_direction(direction) || is_narrower)
                    && !is_loading.get_untracked()
                {
                    set_loading.set(true);

                    let measure = measure.clone();
                    leptos::spawn::spawn_local(async move {
                        #[cfg(debug_assertions)]
                        let zone = SpecialNonReactiveZone::enter();

                        join!(
                            on_load_more.with_value(|f| f(state)),
                            sleep(Duration::from_millis(interval as u64))
                        );

                        #[cfg(debug_assertions)]
                        drop(zone);

                        set_loading.try_set(false);
                        sleep(Duration::ZERO).await;
                        measure();
                        if let Some(check_and_load) = check_and_load.try_get_value().flatten() {
                            check_and_load();
                        }
                    });
                }
            }
        }
    })));

    let _ = watch(
        move || is_element_visible.get(),
        move |visible, prev_visible, _| {
            if *visible && !prev_visible.copied().unwrap_or_default() {
                measure();
            }
        },
        true,
    );

    let _ = watch(
        move || state.arrived_state.get().get_direction(direction),
        move |arrived, prev_arrived, _| {
            if let Some(prev_arrived) = prev_arrived {
                if prev_arrived == arrived {
                    return;
                }
            }

            check_and_load
                .get_value()
                .expect("check_and_load is set above")()
        },
        true,
    );

    is_loading.into()
}

/// Options for [`use_infinite_scroll_with_options`].
#[derive(DefaultBuilder)]
pub struct UseInfiniteScrollOptions {
    /// Callback when scrolling is happening.
    on_scroll: Arc<dyn Fn(web_sys::Event) + Send + Sync>,

    /// Options passed to the `addEventListener("scroll", ...)` call
    event_listener_options: UseEventListenerOptions,

    /// The minimum distance between the bottom of the element and the bottom of the viewport. Default is 0.0.
    distance: f64,

    /// The direction in which to listen the scroll. Defaults to `Direction::Bottom`.
    direction: Direction,

    /// The interval time between two load more (to avoid too many invokes). Default is 100.0.
    interval: f64,
}

impl Default for UseInfiniteScrollOptions {
    fn default() -> Self {
        Self {
            on_scroll: Arc::new(|_| {}),
            event_listener_options: Default::default(),
            distance: 0.0,
            direction: Direction::Bottom,
            interval: 100.0,
        }
    }
}

/// The scroll state being passed into the `on_load_more` callback of [`use_infinite_scroll`].
#[derive(Copy, Clone)]
pub struct ScrollState {
    /// X coordinate of scroll position
    pub x: Signal<f64>,

    /// Y coordinate of scroll position
    pub y: Signal<f64>,

    /// Is true while the element is being scrolled.
    pub is_scrolling: Signal<bool>,

    /// Sets the field that represents a direction to true if the
    /// element is scrolled all the way to that side.
    pub arrived_state: Signal<Directions>,

    /// The directions in which the element is being scrolled are set to true.
    pub directions: Signal<Directions>,
}
