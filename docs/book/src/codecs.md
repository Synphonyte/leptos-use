# Encoding and Decoding Data

Several functions encode and decode data for storing it and/or sending it over the network. To do this, codecs
located at [`src/utils/codecs`](https://github.com/Synphonyte/leptos-use/tree/main/src/utils/codecs) are used. They
implement the traits [`Encoder`](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/mod.rs#L9) with the
method `encode` and [`Decoder`](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/mod.rs#L17) with the
method `decode`.

There are two types of codecs: One that encodes as binary data (`Vec[u8]`) and another type that encodes as
strings (`String`). There is also an adapter
[`Base64`](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/string/base64.rs) that can be used to
wrap a binary codec and make it a string codec by representing the binary data as a base64 string.

## Available Codecs

### String Codecs

- [**`FromToStringCodec`
  **](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/string/from_to_string.rs)
- [**`JsonSerdeCodec`**](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/string/json_serde.rs)**

### Binary Codecs

- [**`FromToBytesCodec`**](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/binary/from_to_bytes.rs)
- [**`BincodeSerdeCodec`**](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/binary/bincode_serde.rs)
- [**`MsgpackSerdeCodec`**](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/binary/msgpack_serde.rs)

### Adapters

- [**`Base64`**](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/string/base64.rs) —
  Wraps a binary codec and make it a string codec by representing the binary data as a base64 string.
- [**`OptionCodec`**](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/option.rs) —
  Wraps a string codec that encodes `T` to create a codec that encodes `Option<T>`.

## Example

In this example, a codec is given to [`use_cookie`](browser/use_cookie.md) that stores data as a string in the JSON
format. Since cookies can only store strings, we have to use string codecs here.

```rust,noplayground
# use leptos::*;
# use leptos_use::use_cookie;
# use serde::{Deserialize, Serialize};

# #[component]
# pub fn App(cx: Scope) -> impl IntoView {
#[derive(Serialize, Deserialize, Clone)]
struct MyState {
    chicken_count: i32,
    egg_count: i32,
}

let (cookie, set_cookie) = use_cookie::<MyState, JsonCodec>("my-state-cookie");    
# view! {}
# }
```

## Custom Codecs

If you don't find a suitable codecs for your needs, you can implement your own; it's straightforward! If you want to
create a string codec, you can look
at [`JsonSerdeCodec`](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/string/json_serde.rs).
In case it's a binary codec, have a look
at [`BincodeSerdeCodec`](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/binary/bincode_serde.rs).

## Versioning

Versioning is the process of handling long-term data that can outlive our code.

For example, we could have a settings struct whose members change over time. We might eventually
add timezone support, and we might then remove support for a thousands separator for numbers.
Each change results in a new possible version of the stored data. If we stored these settings
in browser storage, we would need to handle all possible versions of the data format that can
occur. If we don't offer versioning, then all settings could revert to the default every time we
encounter an old format.

How best to handle versioning depends on the codec involved:

- The `FromToStringCodec` can avoid versioning entirely by keeping
  to primitive types. In our example above, we could have decomposed the settings struct into
  separate timezone and number separator fields. These would be encoded as strings and stored as
  two separate key-value fields in the browser rather than a single field. If a field is missing,
  then the value intentionally would fall back to the default without interfering with the other
  field.

- The `ProstCodec` uses [Protocol buffers](https://protobuf.dev/overview/)
  designed to solve the problem of long-term storage. It provides semantics for versioning that
  are not present in JSON or other formats.

- The codecs that use serde under the hood can rely on serde or by
  providing their own manual version handling. See the next sections for more details.

### Rely on `serde`

A simple way to avoid complex versioning is to rely on serde's [field attributes](https://serde.rs/field-attrs.html)
such as [`serde(default)`](https://serde.rs/field-attrs.html#default)
and [`serde(rename = "...")`](https://serde.rs/field-attrs.html#rename).

### Manual Version Handling

We look at the example of the `JsonSerdeCodec` in this section.

To implement version handling, we parse the JSON generically then transform the
resulting `JsValue` before decoding it into our struct again.

Let's look at an example.

 ```rust,noplayground
 # use leptos::*;
 # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage, UseStorageOptions};
 # use serde::{Deserialize, Serialize};
 # use serde_json::json;
 # use leptos_use::utils::{Encoder, Decoder};
 #
 # pub fn Demo() -> impl IntoView {
 #[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
 pub struct MyState {
     pub hello: String,
     // This field was added in a later version
     pub greeting: String,
 }

 pub struct MyStateCodec;

 impl Encoder<MyState> for MyStateCodec {
     type Error = serde_json::Error;
     type Encoded = String;

     fn encode(val: &MyState) -> Result<Self::Encoded, Self::Error> {
         serde_json::to_string(val)
     }
 }

 impl Decoder<MyState> for MyStateCodec {
     type Error = serde_json::Error;
     type Encoded = str;

     fn decode(stored_value: &Self::Encoded) -> Result<MyState, Self::Error> {
         let mut val: serde_json::Value = serde_json::from_str(stored_value)?;
         // add "greeting": "Hello" to the object if it's missing
         if let Some(obj) = val.as_object_mut() {
             if !obj.contains_key("greeting") {
                obj.insert("greeting".to_string(), json!("Hello"));
             }
             serde_json::from_value(val)
         } else {
             Ok(MyState::default())
         }
     }
 }

 // Then use it like the following just as any other codec.
 let (get, set, remove) = use_local_storage::<MyState, MyStateCodec>("my-struct-key");
 #    view! { }
 # }
 ```
