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

A lot of functions like `use_event_listener` and `use_element_size` are only useful when a target HTML/SVG element is
available. This is not the case on the server. You can simply wrap them in `create_effect` which will cause them to
be only called in the browser.

```rust
create_effect(
    cx,
    move |_| {
        // window() doesn't work on the server
        use_event_listener(cx, window(), "resize", move |_| {
            // ...
        })
    },
);
```