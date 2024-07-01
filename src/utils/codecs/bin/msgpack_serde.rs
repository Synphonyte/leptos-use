use crate::utils::{Decoder, Encoder};

/// A codec that relies on `rmp-serde` to encode data in the msgpack format.
///
/// This is only available with the **`msgpack` feature** enabled.
pub struct MsgpackSerdeCodec;

impl<T: serde::Serialize> Encoder<T> for MsgpackSerdeCodec {
    type Error = rmp_serde::encode::Error;
    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        rmp_serde::to_vec(val)
    }
}

impl<T: serde::de::DeserializeOwned> Decoder<T> for MsgpackSerdeCodec {
    type Error = rmp_serde::decode::Error;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        rmp_serde::from_slice(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msgpack_codec() {
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        struct Test {
            s: String,
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let enc = MsgpackSerdeCodec::encode(&t).unwrap();
        let dec: Test = MsgpackSerdeCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }
}
