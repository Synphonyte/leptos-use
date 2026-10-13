#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Automatically update the height of a textarea depending on the content

mod use_textarea_autosize;
pub use use_textarea_autosize::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_element::*;
    pub use leptos_use_maybe_rw_signal::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_maybe_rw_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_resize_observer::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
