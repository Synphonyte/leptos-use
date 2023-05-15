use crate::use_event_listener;
use leptos::*;

pub struct UseScrollOptions {
    /// Throttle time in milliseconds for the scroll events. Defaults to 0 (disabled).
    pub throttle: u32,

    /// After scrolling ends we wait idle + throttle milliseconds before we consider scrolling to have stopped.
    /// Defaults to 200.
    pub idle: u32,
}

pub struct UseScrollReturn {
    pub x: ReadSignal<f64>,
    pub setX: WriteSignal<f64>,
    pub y: ReadSignal<f64>,
    pub setY: WriteSignal<f64>,
    pub isScrolling: ReadSignal<bool>,
    pub arrivedState: ReadSignal<Directions>,
    pub directions: ReadSignal<Directions>,
}

pub struct Directions {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

pub fn use_scroll() {}
