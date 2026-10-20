# Server-Side Rendering

When using together with server-side rendering (SSR) you have to enable the feature `ssr` similar to
how you do it for `leptos`.

Every Leptos-Use crate that you use (e.g. `leptos-use-mouse`, `leptos-use-color-mode`, …) has an `ssr` feature.
In your Cargo.toml file enable these `ssr` features only from your project's `ssr` feature:

```toml
[dependencies]
leptos-use-color-mode = "0.20"   # do NOT enable the "ssr" feature here
leptos-use-mouse = "0.20"

...

[features]
hydrate = [
    "leptos/hydrate",
    ...
]
ssr = [
    ...
    "leptos/ssr",
    ...
    "leptos-use-color-mode/ssr", # <== add this
    "leptos-use-mouse/ssr",      # <== and this for every leptos-use crate
]

...
```

Please see the [`ssr` example](https://github.com/synphonyte/leptos-use/blob/main/examples/ssr) in the examples folder
for a simple working demonstration.

Many functions work differently on the server and on the client. If that's the case you will
find information about these differences in their respective docs under the section "Server-Side Rendering".
If you don't find that section, it means that the function works exactly the same on both, the client
and the server.

> **Do not enable the `ssr` feature directly**!
>
> Don't do the following.
> ```toml
> [dependencies]
> leptos-use-mouse = { version = "0.20", features = ["ssr"] }  # this is wrong
> ```

The `ssr` feature is used to select which version of the functions are built.
Effectively it means your application is built two times: with `ssr` enabled to
build the server executable, and with `ssr` disabled to build the client's WASM
binary module.

So if you enable a Leptos-Use crate's `ssr` feature globally, you will get the server
version of the functions in the client.

By adding `"leptos-use-mouse/ssr"` to the `ssr` feature of your project, it will only
be enabled when your project is built with `ssr`, and you will get the server
functions server-side, and the client functions client-side.

Functions that read request headers on the server (e.g. `use_cookie`, `use_locales`, `use_color_mode`) additionally
have the features `axum` and `actix` to select your server framework, for example `"leptos-use-cookie/axum"`.

## WASM on the server

If you enable `ssr` in your project on a `wasm32` target architecture, you will get
a compile-time warning in the console because it is a common mistake that users enable `ssr` globally.
If you're using `wasm32` on the server however you can safely disable this warning by
enabling the `wasm_ssr` feature together with `ssr`.

## Functions with Target Elements

A lot of functions like `use_resize_observer` and `use_element_size` are only useful when a target HTML/SVG element is
available. This is not always the case on the server. If you use them with `NodeRefs` they will just work in SSR.
But what if you want to use them with `window()` or `document()`?

To enable that we provide the helper functions [`use_window()`](elements/use_window.md)
and [`use_document()`](elements/use_document.md) which return
a new-type-wrapped `Option<web_sys::Window>` or `Option<web_sys::Document>` respectively. These can be
used safely on the server. The following code works on both the client and the server:

```rust
use leptos::prelude::*;
use leptos::ev::keyup;
use leptos_use_event_listener::use_event_listener;
use leptos_use_window::use_window;

use_event_listener(use_window(), keyup, | evt| {
...
});
```

There are some convenience methods provided as well, like `use_document().body()` which
just propagate a `None` on the server.
