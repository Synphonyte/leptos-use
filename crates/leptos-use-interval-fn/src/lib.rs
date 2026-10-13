#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Wrapper for `set_interval` with controls

mod use_interval_fn;
pub use use_interval_fn::*;

#[allow(unused_imports)]
pub(crate) mod utils {
    pub use leptos_use_filters::*;
    pub use sendwrap_fn::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_filters::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
