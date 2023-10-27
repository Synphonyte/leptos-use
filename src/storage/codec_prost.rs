use super::{Codec, UseStorageOptions};
use base64::Engine;
use thiserror::Error;

#[derive(Clone, Default, PartialEq)]
pub struct ProstCodec();

#[derive(Error, Debug, PartialEq)]
pub enum ProstCodecError {
    #[error("failed to decode base64")]
    DecodeBase64(base64::DecodeError),
    #[error("failed to decode protobuf")]
    DecodeProst(#[from] prost::DecodeError),
}

impl<T: Default + prost::Message> Codec<T> for ProstCodec {
    type Error = ProstCodecError;

    fn encode(&self, val: &T) -> Result<String, Self::Error> {
        let buf = val.encode_to_vec();
        Ok(base64::engine::general_purpose::STANDARD.encode(&buf))
    }

    fn decode(&self, str: String) -> Result<T, Self::Error> {
        let buf = base64::engine::general_purpose::STANDARD
            .decode(str)
            .map_err(ProstCodecError::DecodeBase64)?;
        T::decode(buf.as_slice()).map_err(ProstCodecError::DecodeProst)
    }
}

impl<T: Clone + Default + prost::Message> UseStorageOptions<T, ProstCodec> {
    pub fn prost_codec() -> Self {
        Self::new(ProstCodec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prost_codec() {
        #[derive(Clone, PartialEq, prost::Message)]
        struct Test {
            #[prost(string, tag = "1")]
            s: String,
            #[prost(int32, tag = "2")]
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let codec = ProstCodec();
        assert_eq!(codec.decode(codec.encode(&t).unwrap()), Ok(t));
    }
}
