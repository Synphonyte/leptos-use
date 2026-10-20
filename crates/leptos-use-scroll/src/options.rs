use crate::{Direction, UseEventListenerOptions};
use default_struct_builder::DefaultBuilder;
use leptos::reactive::wrappers::read::Signal;
use std::rc::Rc;

/// Options for [`use_scroll`](crate::use_scroll).
#[derive(DefaultBuilder)]
/// Options for [`use_scroll_with_options`](crate::use_scroll_with_options).
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct UseScrollOptions {
    /// Throttle time in milliseconds for the scroll events. Defaults to 0 (disabled).
    pub(crate) throttle: f64,

    /// After scrolling ends we wait idle + throttle milliseconds before we consider scrolling to have stopped.
    /// Defaults to 200.
    pub(crate) idle: f64,

    /// Threshold in pixels when we consider a side to have arrived (`UseScrollReturn::arrived_state`).
    pub(crate) offset: ScrollOffset,

    /// Callback when scrolling is happening.
    pub(crate) on_scroll: Rc<dyn Fn(web_sys::Event)>,

    /// Callback when scrolling stops (after `idle` + `throttle` milliseconds have passed).
    pub(crate) on_stop: Rc<dyn Fn(web_sys::Event)>,

    /// Options passed to the `addEventListener("scroll", ...)` call
    pub(crate) event_listener_options: UseEventListenerOptions,

    /// When changing the `x` or `y` signals this specifies the scroll behaviour.
    /// Can be `Auto` (= not smooth) or `Smooth`. Defaults to `Auto`.
    #[builder(into)]
    pub(crate) behavior: Signal<ScrollBehavior>,
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
