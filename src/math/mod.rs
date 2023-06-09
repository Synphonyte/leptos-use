#![doc(cfg(feature = "math"))]
//! Collection of reactive math functions

mod shared;
mod use_ceil;
mod use_floor;
mod use_max;
mod use_min;
mod use_round;

pub use use_ceil::*;
pub use use_floor::*;
pub use use_max::*;
pub use use_min::*;
pub use use_round::*;
