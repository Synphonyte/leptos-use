#[cfg(feature = "serde")]
mod codec_json;
#[cfg(feature = "prost")]
mod codec_prost;
mod codec_string;
mod use_local_storage;
mod use_session_storage;
mod use_storage_with_options;

pub use crate::core::StorageType;
#[cfg(feature = "serde")]
pub use codec_json::*;
#[cfg(feature = "prost")]
pub use codec_prost::*;
pub use codec_string::*;
pub use use_local_storage::*;
pub use use_session_storage::*;
pub use use_storage_with_options::*;
