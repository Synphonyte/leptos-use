use crate::utils::{Decoder, Encoder};
use base64::Engine;
use thiserror::Error;

/// Wraps a binary codec and make it a string codec by representing the binary data as a base64
/// string.
///
/// Only available with the **`base64` feature** enabled.
///
/// Example:
///
/// ```
/// # use leptos_use::utils::{Base64, MsgpackSerdeCodec, Encoder, Decoder};
/// # use serde::{Serialize, Deserialize};
/// #
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct MyState {
///     chicken_count: u32,
///     egg_count: u32,
///     farm_name: String,
/// }
///
/// let original_value = MyState {
///     chicken_count: 10,
///     egg_count: 20,
///     farm_name: "My Farm".to_owned(),
/// };
///
/// let encoded: String = Base64::<MsgpackSerdeCodec>::encode(&original_value).unwrap();
/// let decoded: MyState = Base64::<MsgpackSerdeCodec>::decode(&encoded).unwrap();
///
/// assert_eq!(decoded, original_value);
/// ```
pub struct Base64<C>(C);

#[derive(Error, Debug, PartialEq)]
pub enum Base64DecodeError<Err> {
    #[error("failed to decode base64: {0}")]
    DecodeBase64(#[from] base64::DecodeError),
    #[error("failed to decode: {0}")]
    Decoder(Err),
}

impl<T, E> Encoder<T> for Base64<E>
where
    E: Encoder<T, Encoded = Vec<u8>>,
{
    type Error = E::Error;
    type Encoded = String;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        Ok(base64::engine::general_purpose::STANDARD.encode(E::encode(val)?))
    }
}

impl<T, D> Decoder<T> for Base64<D>
where
    D: Decoder<T, Encoded = [u8]>,
{
    type Error = Base64DecodeError<D::Error>;
    type Encoded = str;

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        let buf = base64::engine::general_purpose::STANDARD.decode(val)?;
        D::decode(&buf).map_err(Base64DecodeError::Decoder)
    }
}
