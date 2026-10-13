#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! SSR-friendly and reactive cookie access

mod use_cookie;
pub use use_cookie::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_now::*;
}

#[allow(unused_imports)]
pub(crate) mod utils {
    pub use leptos_use_header::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_broadcast_channel::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_header::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_now::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_watch_with_options::*;
