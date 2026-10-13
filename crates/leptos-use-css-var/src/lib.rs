#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Manipulate CSS variables

mod use_css_var;
pub use use_css_var::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_element::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_watch_with_options::*;
