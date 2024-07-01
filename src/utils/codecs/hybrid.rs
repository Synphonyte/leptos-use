use crate::utils::{Decoder, Encoder};
use thiserror::Error;

pub trait IsBinary<T, E: ?Sized> {
    fn is_binary() -> bool;
}

impl<D, T> IsBinary<T, [u8]> for D
where
    D: Decoder<T, Encoded = [u8]>,
{
    fn is_binary() -> bool {
        true
    }
}

impl<D, T> IsBinary<T, str> for D
where
    D: Decoder<T, Encoded = str>,
{
    fn is_binary() -> bool {
        false
    }
}

#[derive(Debug, Error)]
pub enum HybridCoderError<E> {
    #[error("Not implemented: {0}")]
    NotImplemented(&'static str),
    #[error("Decoding error")]
    Coder(#[from] E),
}

pub trait HybridDecoder<T, E: ?Sized> {
    type Error;

    fn decode_str(_val: &str) -> Result<T, HybridCoderError<Self::Error>> {
        Err(HybridCoderError::NotImplemented(
            "You're trying to decode from a string. This codec is binary.",
        ))
    }

    fn decode_bin(_val: &[u8]) -> Result<T, HybridCoderError<Self::Error>> {
        Err(HybridCoderError::NotImplemented(
            "You're trying to decode from a byte slice. This codec is a string codec.",
        ))
    }
}

impl<T, D> HybridDecoder<T, [u8]> for D
where
    D: Decoder<T, Encoded = [u8]>,
{
    type Error = D::Error;

    fn decode_bin(val: &[u8]) -> Result<T, HybridCoderError<Self::Error>> {
        Ok(D::decode(val)?)
    }
}

impl<T, D> HybridDecoder<T, str> for D
where
    D: Decoder<T, Encoded = str>,
{
    type Error = D::Error;

    fn decode_str(val: &str) -> Result<T, HybridCoderError<Self::Error>> {
        Ok(D::decode(val)?)
    }
}

pub trait HybridEncoder<T, E> {
    type Error;

    fn encode_str(_val: &T) -> Result<String, HybridCoderError<Self::Error>> {
        Err(HybridCoderError::NotImplemented(
            "You're trying to encode into a string. This codec is binary.",
        ))
    }

    fn encode_bin(_val: &T) -> Result<Vec<u8>, HybridCoderError<Self::Error>> {
        Err(HybridCoderError::NotImplemented(
            "You're trying to encode into a byte vec. This codec is a string codec.",
        ))
    }
}

impl<T, E> HybridEncoder<T, Vec<u8>> for E
where
    E: Encoder<T, Encoded = Vec<u8>>,
{
    type Error = E::Error;

    fn encode_bin(val: &T) -> Result<Vec<u8>, HybridCoderError<Self::Error>> {
        Ok(E::encode(val)?)
    }
}

impl<T, E> HybridEncoder<T, String> for E
where
    E: Encoder<T, Encoded = String>,
{
    type Error = E::Error;

    fn encode_str(val: &T) -> Result<String, HybridCoderError<Self::Error>> {
        Ok(E::encode(val)?)
    }
}
