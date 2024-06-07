use crate::utils::{Decoder, Encoder};
use std::str::FromStr;

/// A string codec that relies on [`FromStr`] and [`ToString`]. It can encode anything that
/// implements [`ToString`] and decode anything that implements [`FromStr`].
///
/// This makes simple key / value easy to use for primitive types. It is also useful for encoding
/// simply data structures without depending on third party crates like serde and serde_json.
///
/// ## Example
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage, UseStorageOptions};
/// # use leptos_use::utils::FromToStringCodec;
/// #
/// # pub fn Demo() -> impl IntoView {
/// let (get, set, remove) = use_local_storage::<i32, FromToStringCodec>("my-key");
/// #    view! { }
/// # }
/// ```
pub struct FromToStringCodec;

impl<T: ToString> Encoder<T> for FromToStringCodec {
    type Error = ();
    type Encoded = String;

    fn encode(val: &T) -> Result<String, Self::Error> {
        Ok(val.to_string())
    }
}

impl<T: FromStr> Decoder<T> for FromToStringCodec {
    type Error = T::Err;
    type Encoded = str;

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        T::from_str(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_codec() {
        let s = String::from("party time 🎉");
        assert_eq!(FromToStringCodec::encode(&s), Ok(s.clone()));
        assert_eq!(FromToStringCodec::decode(&s), Ok(s));
    }
}
