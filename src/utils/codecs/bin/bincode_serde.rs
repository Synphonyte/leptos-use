use crate::utils::{Decoder, Encoder};
use serde::{Deserialize, Serialize};

/// A codec that relies on `bincode` adn `serde` to encode data in the bincode format.
///
/// This is only available with the **`bincode` feature** enabled.
pub struct BincodeSerdeCodec;

impl<T: serde::Serialize> Encoder<T> for BincodeSerdeCodec {
    type Error = bincode::Error;
    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        bincode::serialize(val)
    }
}

impl<T: serde::de::DeserializeOwned> Decoder<T> for BincodeSerdeCodec {
    type Error = bincode::Error;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        bincode::deserialize(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bincode_codec() {
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        struct Test {
            s: String,
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let enc = BincodeSerdeCodec::encode(&t).unwrap();
        let dec: Test = BincodeSerdeCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }
}
