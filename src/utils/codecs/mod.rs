mod bin;
mod string;

pub use bin::*;
pub use string::*;
use thiserror::Error;

/// Trait every encoder must implement.
pub trait Encoder<T>: 'static {
    type Error;
    type Encoded;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error>;
}

/// Trait every decoder must implement.
pub trait Decoder<T>: 'static {
    type Error;
    type Encoded: ?Sized;

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error>;
}

/// Trait to check if a type is binary or encodes data in a string.
pub trait IsBinary<T> {
    fn is_binary() -> bool {
        true
    }
}

impl<Enc, T> IsBinary<T> for Enc
where
    Enc: Encoder<T, Encoded = String>,
{
    fn is_binary() -> bool {
        false
    }
}

#[derive(Error, Debug)]
pub enum CodecError<E, D> {
    #[error("failed to encode: {0}")]
    Encode(E),
    #[error("failed to decode: {0}")]
    Decode(D),
}
