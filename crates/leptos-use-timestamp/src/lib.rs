#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive current timestamp

mod use_timestamp;
pub use use_timestamp::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_now::*;
}

#[allow(unused_imports)]
pub(crate) mod utils {
    pub use leptos_use_filters::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_filters::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_interval_fn::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_now::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_raf_fn::*;
