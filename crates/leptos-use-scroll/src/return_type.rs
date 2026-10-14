use crate::Directions;
use leptos::reactive::wrappers::read::Signal;

/// The return value of [`use_scroll`](crate::use_scroll).
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
