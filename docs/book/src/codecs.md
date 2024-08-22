# Encoding and Decoding Data

Several functions encode and decode data for storing it and/or sending it over the network. To do this, codecs
from the crate [`codee`](https://docs.rs/codee/latest/codee/) are used. They
implement the traits [`Encoder`](https://docs.rs/codee/latest/codee/trait.Encoder.html) with the
method `encode` and [`Decoder`](https://docs.rs/codee/latest/codee/trait.Decoder.html) with the
method `decode`.

There are two types of codecs: One that encodes as binary data (`Vec[u8]`) and another type that encodes as
strings (`String`). There is also an adapter
[`Base64`](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/string/base64.rs) that can be used to
wrap a binary codec and make it a string codec by representing the binary data as a base64 string.

Please check the documentation of [`codee`](https://docs.rs/codee/latest/codee/) for more details and a list of all
available codecs.

## Example

In this example, a codec is given to [`use_cookie`](browser/use_cookie.md) that stores data as a string in the JSON
format. Since cookies can only store strings, we have to use string codecs here.

```rust,noplayground
# use leptos::*;
# use leptos_use::use_cookie;
# use serde::{Deserialize, Serialize};
# use codee::string::JsonCodec;

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

If you don't find a suitable codec for your needs, you can implement your own; it's straightforward!
If you want to create a string codec, you can look at
[`JsonSerdeCodec`](https://docs.rs/codee/latest/src/codee/string/json_serde.rs.html).
In case it's a binary codec, have a look at
[`BincodeSerdeCodec`](https://docs.rs/codee/latest/src/codee/binary/bincode_serde.rs.html).

## Versioning

For a discussion on how to implement versioning please refer to the
[relevant section in the docs for `codee`](https://docs.rs/codee/latest/codee/index.html#versioning).