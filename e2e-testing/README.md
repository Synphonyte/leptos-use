# End to End testing of leptos-use hooks

The library [`leptos-use`](https://github.com/Synphonyte/leptos-use) provides a collection of useful
hooks for interaction with browser APIs in Leptos applications, namely [`web_sys`](https://crates.io/crates/web-sys),
which does not fulfill Send + Sync traits. There these hooks have to provide threat safety to enable
compatibility with `Server Side Rendering (SSR)`. This testing crate is focused on testing
threat safety with `SSR` for `leptos-use` hooks.

## Running the tests

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos --locked
```

Then run
```bash
cargo leptos end-to-end
```

or

```bash
cargo leptos end-to-end --release
```

to run the end-to-end tests.
