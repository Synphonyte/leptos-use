#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Read request headers during SSR (axum / actix) for the leptos-use crates

#[allow(unused_imports)]
pub(crate) mod utils {
    #[cfg(all(feature = "ssr", any(feature = "axum", feature = "actix")))]
    pub(crate) mod header;
    pub(crate) mod header_macro;
    #[cfg(all(feature = "ssr", any(feature = "axum", feature = "actix")))]
    pub use header::*;
    pub use header_macro::*;
}
#[cfg(all(feature = "ssr", any(feature = "axum", feature = "actix")))]
#[allow(unused_imports)]
pub use utils::header::*;
#[allow(unused_imports)]
pub use utils::header_macro::*;
