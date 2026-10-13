#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Macro to wrap closures in a `send_wrapper::SendWrapper` so they are `Send + Sync`

#[allow(unused_imports)]
pub(crate) mod utils {
    pub(crate) mod sendwrap_fn;
    pub use sendwrap_fn::*;
}
#[allow(unused_imports)]
pub use utils::sendwrap_fn::*;
