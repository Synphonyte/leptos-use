# Get Started

## Installation

```shell
cargo add leptos-use
```

## Examples

- [Examples Directory](https://github.com/Synphonyte/leptos-use/tree/main/examples)

## Usage Example

Simply import the functions you need from `leptos-use`

```rust,noplayground
use leptos::*;
use leptos_use::{use_mouse, UseMouseReturn};

#[component]
fn Demo() -> impl IntoView {
    let UseMouseReturn { x, y, .. } = use_mouse();
    
    view! { cx,
        {x} " x " {y}
    }
}
```

Please refer to the [functions list](functions.md) for more details.

## Stable Rust

By default — like `leptos` — the library assumes you're using the
nightly Rust toolchain. This allows for more ergonomic use of signals.
If you want to use stable Rust, you have to enable the `stable` crate feature.

```toml
leptos-use = { version = "...", features = ["stable"] }
```
