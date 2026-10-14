use crate::{Direction, UseEventListenerOptions};
use default_struct_builder::DefaultBuilder;
use std::sync::Arc;

/// Options for [`use_infinite_scroll_with_options`](crate::use_infinite_scroll_with_options).
#[derive(DefaultBuilder)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct UseInfiniteScrollOptions {
    /// Callback when scrolling is happening.
    pub(crate) on_scroll: Arc<dyn Fn(web_sys::Event) + Send + Sync>,

    /// Options passed to the `addEventListener("scroll", ...)` call
    pub(crate) event_listener_options: UseEventListenerOptions,

    /// The minimum distance between the bottom of the element and the bottom of the viewport. Default is 0.0.
    pub(crate) distance: f64,

    /// The direction in which to listen the scroll. Defaults to `Direction::Bottom`.
    pub(crate) direction: Direction,

    /// The interval time between two load more (to avoid too many invokes). Default is 100.0.
    pub(crate) interval: f64,
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
