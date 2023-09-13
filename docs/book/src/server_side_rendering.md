# Server-Side Rendering

When using together with server-side rendering (SSR) you have to enable the feature `ssr` similar to
how you do it for `leptos`.

In your Cargo.toml file add the following:

```toml
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
    "leptos-use/ssr" # add this
]

...
```

Please see the `ssr` example in the examples folder for a simple working demonstration.

Many functions work differently on the server and on the client. If that's the case you will
find information about these differences in their respective docs under the section "Server-Side Rendering".
If you don't find that section, it means that the function works exactly the same on both, the client
and the server.

## Functions with Target Elements

A lot of functions like `use_resize_observer` and `use_element_size` are only useful when a target HTML/SVG element is
available. This is not always the case on the server. If you use them with `NodeRefs` they will just work in SSR.
But what if you want to use them with `window()` or `document()`?

To enable that we provide the helper functions [`use_window()`](elements/use_window.md) and [`use_document()`](elements/use_document.md) which return
a new-type-wrapped `Option<web_sys::Window>` or `Option<web_sys::Document>` respectively. These can be
used safely on the server. The following code works on both the client and the server:

```rust
use leptos::*;
use leptos::ev::keyup;
use leptos_use::{use_event_listener, use_window};

use_event_listener(use_window(), keyup, |evt| {
    ...
}); 
```

There are some convenience methods provided as well, like `use_document().body()` which
just propagate a `None` on the server.