#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Create bare-bone calendar data to use in your component

mod use_calendar;
pub use use_calendar::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_maybe_rw_signal::*;
    pub use leptos_use_now::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_maybe_rw_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_now::*;
