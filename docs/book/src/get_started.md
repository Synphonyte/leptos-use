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
fn Demo(cx: Scope) -> impl IntoView {
    let UseMouseReturn { x, y, .. } = use_mouse(cx);
    
    view! { cx,
        {x} " x " {y}
    }
}
```

Please refer to the [functions list](functions.md) for more details.