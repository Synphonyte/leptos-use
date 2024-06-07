use crate::utils::{Decoder, Encoder};

/// A codec for storing ProtoBuf messages that relies on [`prost`](https://github.com/tokio-rs/prost) to parse.
///
/// [Protocol buffers](https://protobuf.dev/overview/) is a serialisation format useful for
/// long-term storage. It provides semantics for versioning that are not present in JSON or other
/// formats. [`prost`] is a Rust implementation of Protocol Buffers.
///
/// This codec uses [`prost`](https://github.com/tokio-rs/prost) to encode the message into a byte stream.
/// To use it with local storage in the example below we wrap it with [`Base64`] to represent the bytes as a string.
///
/// ## Example
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage, UseStorageOptions};
/// # use leptos_use::utils::{Base64, ProstCodec};
/// #
/// # pub fn Demo() -> impl IntoView {
/// // Primitive types:
/// let (get, set, remove) = use_local_storage::<i32, Base64<ProstCodec>>("my-key");
///
/// // Structs:
/// #[derive(Clone, PartialEq, prost::Message)]
/// pub struct MyState {
///     #[prost(string, tag = "1")]
///     pub hello: String,
/// }
/// let (get, set, remove) = use_local_storage::<MyState, Base64<ProstCodec>>("my-struct-key");
/// #    view! { }
/// # }
/// ```
///
/// Note: we've defined and used the `prost` attribute here for brevity. Alternate usage would be to
/// describe the message in a .proto file and use [`prost_build`](https://docs.rs/prost-build) to
/// auto-generate the Rust code.
pub struct ProstCodec;

impl<T: prost::Message> Encoder<T> for ProstCodec {
    type Error = ();
    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        let buf = val.encode_to_vec();
        Ok(buf)
    }
}

impl<T: prost::Message + Default> Decoder<T> for ProstCodec {
    type Error = prost::DecodeError;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        T::decode(val)
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
        assert_eq!(ProstCodec::decode(&ProstCodec::encode(&t).unwrap()), Ok(t));
    }
}
