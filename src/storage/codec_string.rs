use super::{Codec, UseStorageOptions};
use std::str::FromStr;

#[derive(Clone, Default, PartialEq)]
pub struct StringCodec();

impl<T: FromStr + ToString> Codec<T> for StringCodec {
    type Error = T::Err;

    fn encode(&self, val: &T) -> Result<String, Self::Error> {
        Ok(val.to_string())
    }

    fn decode(&self, str: String) -> Result<T, Self::Error> {
        T::from_str(&str)
    }
}

impl<T: Clone + Default + FromStr + ToString> UseStorageOptions<T, StringCodec> {
    pub fn string_codec() -> Self {
        Self::new(StringCodec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_codec() {
        let s = String::from("party time 🎉");
        let codec = StringCodec();
        assert_eq!(codec.encode(&s), Ok(s.clone()));
        assert_eq!(codec.decode(s.clone()), Ok(s));
    }
}
