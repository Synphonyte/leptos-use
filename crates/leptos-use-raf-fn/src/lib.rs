#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Call function on every requestAnimationFrame

mod use_raf_fn;
pub use use_raf_fn::*;

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
