use super::{Codec, UseStorageOptions};

#[derive(Clone, PartialEq)]
pub struct JsonCodec();

impl<T: serde::Serialize + serde::de::DeserializeOwned> Codec<T> for JsonCodec {
    type Error = serde_json::Error;

    fn encode(&self, val: &T) -> Result<String, Self::Error> {
        serde_json::to_string(val)
    }

    fn decode(&self, str: String) -> Result<T, Self::Error> {
        serde_json::from_str(&str)
    }
}

impl<T: Clone + Default + serde::Serialize + serde::de::DeserializeOwned>
    UseStorageOptions<T, JsonCodec>
{
    pub fn json_codec() -> Self {
        Self::new(JsonCodec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_codec() {
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        struct Test {
            s: String,
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let codec = JsonCodec();
        let enc = codec.encode(&t).unwrap();
        let dec: Test = codec.decode(enc).unwrap();
        assert_eq!(dec, t);
    }
}
