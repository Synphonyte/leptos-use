#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Debounce/throttle/pausable filters used by the leptos-use crates

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_now::*;
}

#[allow(unused_imports)]
pub(crate) mod utils {
    pub(crate) mod filters;
    pub(crate) mod pausable;
    pub use filters::*;
    pub use pausable::*;
    pub use sendwrap_fn::*;
}
#[allow(unused_imports)]
pub use utils::filters::*;
#[allow(unused_imports)]
pub use utils::pausable::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_now::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
