#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive color mode (dark / light / customs) with auto data persistence

mod use_color_mode;
pub use use_color_mode::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub(crate) mod url;
    pub use leptos_use_element::*;
    pub use leptos_use_maybe_rw_signal::*;
    pub use url::*;
}

#[allow(unused_imports)]
pub(crate) mod utils {
    pub use leptos_use_header::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_cookie::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_header::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_maybe_rw_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_preferences::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_storage as storage;
#[allow(unused_imports)]
pub(crate) use leptos_use_storage::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_sync_signal::*;
