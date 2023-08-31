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

Just like `leptos` this library can be safely run on stable rust. Just don't
forget to consult the Getting Started section of the `leptos` docs to understand
what that means. As of writing, it concerns mainly the getter and setter syntax
on signals, and the use of `cx` in closures.
