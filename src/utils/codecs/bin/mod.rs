mod from_to_bytes;

#[allow(unused_imports)]
pub use from_to_bytes::*;

/// A codec for encoding and decoding values to and from strings.
/// These strings are intended to be  sent over the network.
pub trait BinCodec<T>: Clone + 'static {
    /// The error type returned when encoding or decoding fails.
    type Error;
    /// Encodes a value to a string.
    fn encode(&self, val: &T) -> Result<Vec<u8>, Self::Error>;
    /// Decodes a string to a value. Should be able to decode any string encoded by [`encode`].
    fn decode(&self, val: &[u8]) -> Result<T, Self::Error>;
}
