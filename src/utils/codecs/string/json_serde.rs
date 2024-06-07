use crate::utils::{Decoder, Encoder};

/// A codec for encoding JSON messages that relies on [`serde_json`].
///
/// Only available with the **`json` feature** enabled.
///
/// ## Example
///
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage, UseStorageOptions};
/// # use serde::{Deserialize, Serialize};
/// # use leptos_use::utils::JsonSerdeCodec;
/// #
/// # pub fn Demo() -> impl IntoView {
/// // Primitive types:
/// let (get, set, remove) = use_local_storage::<i32, JsonSerdeCodec>("my-key");
///
/// // Structs:
/// #[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
/// pub struct MyState {
///     pub hello: String,
/// }
/// let (get, set, remove) = use_local_storage::<MyState, JsonSerdeCodec>("my-struct-key");
/// #    view! { }
/// # }
/// ```
pub struct JsonSerdeCodec;

impl<T: serde::Serialize> Encoder<T> for JsonSerdeCodec {
    type Error = serde_json::Error;
    type Encoded = String;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        serde_json::to_string(val)
    }
}

impl<T: serde::de::DeserializeOwned> Decoder<T> for JsonSerdeCodec {
    type Error = serde_json::Error;
    type Encoded = str;

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        serde_json::from_str(val)
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
        let enc = JsonSerdeCodec::encode(&t).unwrap();
        let dec: Test = JsonSerdeCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }
}
