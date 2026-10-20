# Get Started

## Installation

Every function lives in its own small crate. Add the ones you need, for example:

```shell
cargo add leptos-use-mouse
```

Each function's page in this book tells you which crate to add.

> The all-in-one `leptos-use` crate still exists but only re-exports the micro crates with deprecation notes.
> It will not receive new functionality.

## Examples

- [Examples Directory](https://github.com/Synphonyte/leptos-use/tree/main/examples)

## Usage Example

Simply import the functions you need from their crates

```rust,noplayground
use leptos::prelude::*;
use leptos_use_mouse::{use_mouse, UseMouseReturn};

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

Just like `leptos` this library can be safely run on stable rust.
In the [Getting Started section](https://book.leptos.dev/getting_started/index.html)
of the `leptos` docs you can read more about what this means.
