use super::BinCodec;

#[derive(Copy, Clone, Default, PartialEq)]
pub struct FromToBytesCodec;

#[derive(Error, Debug, PartialEq)]
pub enum FromToBytesCodecError {
    #[error("failed to convert byte slice to byte array")]
    InvalidByteSlice(#[from] std::array::TryFromSliceError),

    #[error("failed to convert byte array to string")]
    InvalidString(#[from] std::string::FromUtf8Error),
}

macro_rules! impl_bin_codec_for_number {
    ($num:ty) => {
        impl BinCodec<$num> for FromToBytesCodec {
            type Error = FromToBytesCodecError;

            fn encode(&self, val: &$num) -> Result<Vec<u8>, Self::Error> {
                Ok(val.to_be_bytes().to_vec())
            }

            fn decode(&self, val: &[u8]) -> Result<$num, Self::Error> {
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

impl BinCodec<bool> for FromToBytesCodec {
    type Error = FromToBytesCodecError;

    fn encode(&self, val: &bool) -> Result<Vec<u8>, Self::Error> {
        let codec = FromToBytesCodec;
        let num: u8 = if *val { 1 } else { 0 };
        codec.encode(&num)
    }

    fn decode(&self, val: &[u8]) -> Result<bool, Self::Error> {
        let codec = FromToBytesCodec;
        let num: u8 = codec.decode(val)?;
        Ok(num != 0)
    }
}

impl BinCodec<String> for FromToBytesCodec {
    type Error = FromToBytesCodecError;

    fn encode(&self, val: &String) -> Result<Vec<u8>, Self::Error> {
        Ok(val.as_bytes().to_vec())
    }

    fn decode(&self, val: &[u8]) -> Result<String, Self::Error> {
        Ok(String::from_utf8(val.to_vec())?)
    }
}
