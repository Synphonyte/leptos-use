#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Listen for clicks outside an element

mod on_click_outside;
pub use on_click_outside::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_element::*;
}

#[allow(unused_imports)]
pub(crate) mod utils {
    pub(crate) mod is;
    pub use is::*;
    pub use sendwrap_fn::*;
}
#[allow(unused_imports)]
pub use utils::is::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
