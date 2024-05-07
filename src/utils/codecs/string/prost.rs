use super::StringCodec;
use base64::Engine;
use thiserror::Error;

/// A codec for storing ProtoBuf messages that relies on [`prost`] to parse.
///
/// [Protocol buffers](https://protobuf.dev/overview/) is a serialisation format useful for long-term storage. It provides semantics for versioning that are not present in JSON or other formats. [`prost`] is a Rust implementation of Protocol Buffers.
///
/// This codec uses [`prost`] to encode the message and then [`base64`](https://docs.rs/base64) to represent the bytes as a string.
///
/// ## Example
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage, UseStorageOptions};
/// # use leptos_use::utils::ProstCodec;
/// #
/// # pub fn Demo() -> impl IntoView {
/// // Primitive types:
/// let (get, set, remove) = use_local_storage::<i32, ProstCodec>("my-key");
///
/// // Structs:
/// #[derive(Clone, PartialEq, prost::Message)]
/// pub struct MyState {
///     #[prost(string, tag = "1")]
///     pub hello: String,
/// }
/// let (get, set, remove) = use_local_storage::<MyState, ProstCodec>("my-struct-key");
/// #    view! { }
/// # }
/// ```
///
/// Note: we've defined and used the `prost` attribute here for brevity. Alternate usage would be to describe the message in a .proto file and use [`prost_build`](https://docs.rs/prost-build) to auto-generate the Rust code.
#[derive(Copy, Clone, Default, PartialEq)]
pub struct ProstCodec;

#[derive(Error, Debug, PartialEq)]
pub enum ProstCodecError {
    #[error("failed to decode base64")]
    DecodeBase64(base64::DecodeError),
    #[error("failed to decode protobuf")]
    DecodeProst(#[from] prost::DecodeError),
}

impl<T: Default + prost::Message> StringCodec<T> for ProstCodec {
    type Error = ProstCodecError;

    fn encode(&self, val: &T) -> Result<String, Self::Error> {
        let buf = val.encode_to_vec();
        Ok(base64::engine::general_purpose::STANDARD.encode(buf))
    }

    fn decode(&self, str: String) -> Result<T, Self::Error> {
        let buf = base64::engine::general_purpose::STANDARD
            .decode(str)
            .map_err(ProstCodecError::DecodeBase64)?;
        T::decode(buf.as_slice()).map_err(ProstCodecError::DecodeProst)
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
        let codec = ProstCodec;
        assert_eq!(codec.decode(codec.encode(&t).unwrap()), Ok(t));
    }
}
