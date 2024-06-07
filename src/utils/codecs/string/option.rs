use crate::utils::{Decoder, Encoder};

/// Wraps a string codec that encodes `T` to create a codec that encodes `Option<T>`.
///
/// Example:
///
/// ```
/// # use leptos_use::utils::{OptionCodec, FromToStringCodec, Encoder, Decoder};
/// #
/// let original_value = Some(4);
/// let encoded = OptionCodec::<FromToStringCodec>::encode(&original_value).unwrap();
/// let decoded = OptionCodec::<FromToStringCodec>::decode(&encoded).unwrap();
///
/// assert_eq!(decoded, original_value);
/// ```
pub struct OptionCodec<C>(C);

impl<T, E> Encoder<Option<T>> for OptionCodec<E>
where
    E: Encoder<T, Encoded = String>,
{
    type Error = E::Error;
    type Encoded = String;

    fn encode(val: &Option<T>) -> Result<String, Self::Error> {
        match val {
            Some(val) => Ok(format!("~<|Some|>~{}", E::encode(val)?)),
            None => Ok("~<|None|>~".to_owned()),
        }
    }
}

impl<T, D> Decoder<Option<T>> for OptionCodec<D>
where
    D: Decoder<T, Encoded = str>,
{
    type Error = D::Error;
    type Encoded = str;

    fn decode(str: &Self::Encoded) -> Result<Option<T>, Self::Error> {
        str.strip_prefix("~<|Some|>~")
            .map(|v| D::decode(v))
            .transpose()
    }
}
