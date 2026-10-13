#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Small shared types (Position, Size, Direction, …) used by the leptos-use crates

#[allow(unused_imports)]
pub(crate) mod core {
    pub(crate) mod connection_ready_state;
    pub(crate) mod direction;
    pub(crate) mod pointer_type;
    pub(crate) mod position;
    pub(crate) mod reconnect_limit;
    pub(crate) mod size;
    pub use connection_ready_state::*;
    pub use direction::*;
    pub use pointer_type::*;
    pub use position::*;
    pub use reconnect_limit::*;
    pub use size::*;
}
#[allow(unused_imports)]
pub use core::connection_ready_state::*;
#[allow(unused_imports)]
pub use core::direction::*;
#[allow(unused_imports)]
pub use core::pointer_type::*;
#[allow(unused_imports)]
pub use core::position::*;
#[allow(unused_imports)]
pub use core::reconnect_limit::*;
#[allow(unused_imports)]
pub use core::size::*;
