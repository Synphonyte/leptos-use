mod from_to_string;
#[cfg(feature = "serde_json")]
mod json;
#[cfg(feature = "prost")]
mod prost;

pub use from_to_string::*;
#[cfg(feature = "serde_json")]
pub use json::*;
#[cfg(feature = "prost")]
pub use prost::*;

/// A codec for encoding and decoding values to and from strings.
/// These strings are intended to be stored in browser storage or sent over the network.
pub trait StringCodec<T>: Clone + 'static {
    /// The error type returned when encoding or decoding fails.
    type Error;
    /// Encodes a value to a string.
    fn encode(&self, val: &T) -> Result<String, Self::Error>;
    /// Decodes a string to a value. Should be able to decode any string encoded by [`encode`].
    fn decode(&self, str: String) -> Result<T, Self::Error>;
}
