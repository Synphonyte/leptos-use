use super::StringCodec;

/// A codec for storing JSON messages that relies on [`serde_json`] to parse.
///
/// ## Example
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage, UseStorageOptions, JsonCodec};
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
///
/// ## Versioning
///
/// If the JSON decoder fails, the storage hook will return `T::Default` dropping the stored JSON value. See [`Codec`](super::Codec) for general information on codec versioning.
///
/// ### Rely on serde
/// This codec uses [`serde_json`] under the hood. A simple way to avoid complex versioning is to rely on serde's [field attributes](https://serde.rs/field-attrs.html) such as [`serde(default)`](https://serde.rs/field-attrs.html#default) and [`serde(rename = "...")`](https://serde.rs/field-attrs.html#rename).
///
/// ### String replacement
/// Previous versions of leptos-use offered a `merge_defaults` fn to rewrite the encoded value. This is possible by wrapping the codec but should be avoided.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage, UseStorageOptions, Codec, JsonCodec};
/// # use serde::{Deserialize, Serialize};
/// #
/// # pub fn Demo() -> impl IntoView {
/// #[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
/// pub struct MyState {
///     pub hello: String,
///     pub greeting: String,
/// }
///
/// #[derive(Clone, Default)]
/// pub struct MyStateCodec();
/// impl Codec<MyState> for MyStateCodec {
///     type Error = serde_json::Error;
///
///     fn encode(&self, val: &MyState) -> Result<String, Self::Error> {
///         serde_json::to_string(val)
///     }
///
///     fn decode(&self, stored_value: String) -> Result<MyState, Self::Error> {
///         let default_value = MyState::default();
///         let rewritten = if stored_value.contains(r#""greeting":"#) {
///             stored_value
///         } else {
///             // add "greeting": "Hello" to the string
///             stored_value.replace("}", &format!(r#""greeting": "{}"}}"#, default_value.greeting))
///         };
///         serde_json::from_str(&rewritten)
///     }
/// }
///
/// let (get, set, remove) = use_local_storage::<MyState, MyStateCodec>("my-struct-key");
/// #    view! { }
/// # }
/// ```
///
/// ### Transform a `JsValue`
/// A better alternative to string replacement might be to parse the JSON then transform the resulting `JsValue` before decoding it to to your struct again.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage, UseStorageOptions, Codec, JsonCodec};
/// # use serde::{Deserialize, Serialize};
/// # use serde_json::json;
/// #
/// # pub fn Demo() -> impl IntoView {
/// #[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
/// pub struct MyState {
///     pub hello: String,
///     pub greeting: String,
/// }
///
/// #[derive(Clone, Default)]
/// pub struct MyStateCodec();
/// impl Codec<MyState> for MyStateCodec {
///     type Error = serde_json::Error;
///
///     fn encode(&self, val: &MyState) -> Result<String, Self::Error> {
///         serde_json::to_string(val)
///     }
///
///     fn decode(&self, stored_value: String) -> Result<MyState, Self::Error> {
///         let mut val: serde_json::Value = serde_json::from_str(&stored_value)?;
///         // add "greeting": "Hello" to the object if it's missing
///         if let Some(obj) = val.as_object_mut() {
///             if !obj.contains_key("greeting") {
///                obj.insert("greeting".to_string(), json!("Hello"));
///             }
///             serde_json::from_value(val)
///         } else {
///             Ok(MyState::default())
///         }
///     }
/// }
///
/// let (get, set, remove) = use_local_storage::<MyState, MyStateCodec>("my-struct-key");
/// #    view! { }
/// # }
/// ```
#[derive(Copy, Clone, Default, PartialEq)]
pub struct JsonCodec;

impl<T: serde::Serialize + serde::de::DeserializeOwned> StringCodec<T> for JsonCodec {
    type Error = serde_json::Error;

    fn encode(&self, val: &T) -> Result<String, Self::Error> {
        serde_json::to_string(val)
    }

    fn decode(&self, str: String) -> Result<T, Self::Error> {
        serde_json::from_str(&str)
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
        let codec = JsonCodec;
        let enc = codec.encode(&t).unwrap();
        let dec: Test = codec.decode(enc).unwrap();
        assert_eq!(dec, t);
    }
}
