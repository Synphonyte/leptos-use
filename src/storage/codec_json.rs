use super::{Codec, UseStorageOptions};

/// A codec for storing JSON messages that relies on [`serde_json`] to parse.
///
/// ## Example
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage_with_options, UseStorageOptions, StringCodec, JsonCodec, ProstCodec};
/// # use serde::{Deserialize, Serialize};
/// #
/// # pub fn Demo() -> impl IntoView {
/// // Primitive types:
/// let (get, set, remove) = use_local_storage::<i32, JsonCodec>("my-key");
///
/// // Structs:
/// #[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
/// pub struct MyState {
///     pub hello: String,
/// }
/// let (get, set, remove) = use_local_storage::<MyState, JsonCodec>("my-struct-key");
/// #    view! { }
/// # }
/// ```
#[derive(Clone, Default, PartialEq)]
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
    /// Constructs a new `UseStorageOptions` with a [`JsonCodec`] for JSON messages.
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
