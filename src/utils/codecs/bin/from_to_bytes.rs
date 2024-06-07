use crate::utils::{Decoder, Encoder};
use thiserror::Error;

/// A binary codec that uses rust own binary encoding functions to encode and decode data.
/// This can be used if you want to encode only primitives and don't want to rely on third party
/// crates like `bincode` or `rmp-serde`. If you have more complex data check out
/// [`BincodeSerdeCodec`] or [`MsgpackSerdeCodec`].
pub struct FromToBytesCodec;

#[derive(Error, Debug)]
pub enum FromToBytesCodecError {
    #[error("failed to convert byte slice to byte array")]
    InvalidByteSlice(#[from] std::array::TryFromSliceError),

    #[error("failed to convert byte array to string")]
    InvalidString(#[from] std::string::FromUtf8Error),
}

macro_rules! impl_bin_codec_for_number {
    ($num:ty) => {
        impl Encoder<$num> for FromToBytesCodec {
            type Error = ();
            type Encoded = Vec<u8>;

            fn encode(val: &$num) -> Result<Self::Encoded, Self::Error> {
                Ok(val.to_be_bytes().to_vec())
            }
        }

        impl Decoder<$num> for FromToBytesCodec {
            type Error = FromToBytesCodecError;
            type Encoded = [u8];

            fn decode(val: &Self::Encoded) -> Result<$num, Self::Error> {
                Ok(<$num>::from_be_bytes(val.try_into()?))
            }
        }
    };
}

impl_bin_codec_for_number!(i8);
impl_bin_codec_for_number!(u8);

impl_bin_codec_for_number!(i16);
impl_bin_codec_for_number!(u16);

impl_bin_codec_for_number!(i32);
impl_bin_codec_for_number!(u32);

impl_bin_codec_for_number!(i64);
impl_bin_codec_for_number!(u64);

impl_bin_codec_for_number!(i128);
impl_bin_codec_for_number!(u128);

impl_bin_codec_for_number!(isize);
impl_bin_codec_for_number!(usize);

impl_bin_codec_for_number!(f32);
impl_bin_codec_for_number!(f64);

impl Encoder<bool> for FromToBytesCodec {
    type Error = ();
    type Encoded = Vec<u8>;

    fn encode(val: &bool) -> Result<Self::Encoded, Self::Error> {
        let num: u8 = if *val { 1 } else { 0 };
        Self::encode(&num)
    }
}

impl Decoder<bool> for FromToBytesCodec {
    type Error = FromToBytesCodecError;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<bool, Self::Error> {
        let num: u8 = Self::decode(val)?;
        Ok(num != 0)
    }
}

impl Encoder<String> for FromToBytesCodec {
    type Error = ();
    type Encoded = Vec<u8>;

    fn encode(val: &String) -> Result<Self::Encoded, Self::Error> {
        Ok(val.as_bytes().to_vec())
    }
}

impl Decoder<String> for FromToBytesCodec {
    type Error = FromToBytesCodecError;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<String, Self::Error> {
        Ok(String::from_utf8(val.to_vec())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fromtobytes_codec() {
        let t = 50;

        let enc: Vec<u8> = FromToBytesCodec::encode(&t).unwrap();
        let dec: i32 = FromToBytesCodec::decode(enc.as_slice()).unwrap();
        assert_eq!(dec, t);
    }
}
