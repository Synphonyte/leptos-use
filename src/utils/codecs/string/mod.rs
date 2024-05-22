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
///
/// ## Versioning
///
/// Versioning is the process of handling long-term data that can outlive our code.
///
/// For example we could have a settings struct whose members change over time. We might eventually add timezone support and we might then remove support for a thousand separator on numbers. Each change results in a new possible version of the stored data. If we stored these settings in browser storage we would need to handle all possible versions of the data format that can occur. If we don't offer versioning then all settings could revert to the default every time we encounter an old format.
///
/// How best to handle versioning depends on the codec involved:
///
/// - The [`StringCodec`](super::StringCodec) can avoid versioning entirely by keeping to privimitive types. In our example above, we could have decomposed the settings struct into separate timezone and number separator fields. These would be encoded as strings and stored as two separate key-value fields in the browser rather than a single field. If a field is missing then the value intentionally would fallback to the default without interfering with the other field.
///
/// - The [`ProstCodec`](super::ProstCodec) uses [Protocol buffers](https://protobuf.dev/overview/) designed to solve the problem of long-term storage. It provides semantics for versioning that are not present in JSON or other formats.
///
/// - The [`JsonCodec`](super::JsonCodec) stores data as JSON. We can then rely on serde or by providing our own manual version handling. See the codec for more details.
pub trait StringCodec<T>: Clone + 'static {
    /// The error type returned when encoding or decoding fails.
    type Error: Clone;
    /// Encodes a value to a string.
    fn encode(&self, val: &T) -> Result<String, Self::Error>;
    /// Decodes a string to a value. Should be able to decode any string encoded by [`encode`].
    fn decode(&self, str: String) -> Result<T, Self::Error>;
}
