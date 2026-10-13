#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Creating and managing a [Websocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket) connection

mod use_websocket;
pub use use_websocket::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_core::*;
}

#[allow(unused_imports)]
pub(crate) mod utils {
    pub use leptos_use_filters::*;
    pub use sendwrap_fn::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_core::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_filters::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_interval_fn::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
