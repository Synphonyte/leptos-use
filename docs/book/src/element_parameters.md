# Element Parameters

Many functions in this library operate on HTML/SVG elements. For example, the
function [`use_element_size`](elements/use_element_size.md) returns the width and height of an element:

```rust
# use leptos::{*, html::Div};
# use leptos_use::{use_element_size, UseElementSizeReturn};
#
# #[component]
# pub fn Component() -> impl IntoView {
let el = create_node_ref::<Div>();

let UseElementSizeReturn { width, height } = use_element_size(el);

view! {
    <div node_ref=el></div>
}
# }
```

In the example above we used a Leptos `NodeRef` to pass into the function. But that is not
the only way you can do that. All of these work as well:

```rust
use_element_size(window().body()); // Option<web_sys::Element>
use_element_size(window().body().unwrap()); // web_sys::Element
use_element_size("div > p.some-class"); // &str or String intepreted as CSS selector

pub fn some_directive(el: HtmlElement<AnyElement>) {
    use_element_size(el); // leptos::html::HtmlElement<T>
}
```

Signal of Strings: `Signal<String>`, `ReadSignal<String>`, `RwSignal<String>`, `Memo<String>`; also works with `&str`:

```rust
let (str_signal, set_str_signal) = signal("div > p.some-class".to_string());
use_element_size(str_signal);
```

Signals of
Elements: `Signal<web_sys::Element>`, `ReadSignal<web_sys::Element>`, `RwSignal<web_sys::Element>`, `Memo<web_sys::Element>`;
also works with `Option<web_sys::Element>`:

```rust
let (el_signal, set_el_signal) = signal(document().query_selector("div > p.some-class").unwrap());
use_element_size(el_signal); 
```

## How it works

Looking at the source code of `use_element_size` you'll find sth like

```rust
pub fn use_element_size(el: Into<ElementMaybeSignal<...>>) -> UseElementSizeReturn {}
```

All the above code works because there are `From` implementations for all of these
types for `ElementMaybeSignal`.

## `ElementsMaybeSignal`

Some functions work on one or more elements. Take [`use_resize_observer`](elements/use_resize_observer.md) for example.
This works very much the same way as described above but instead of `Into<ElementMaybeSignal>`
it takes an `Into<ElementsMaybeSignal>` (note the plural). This means you can use it exactly in
the same ways as you saw with the singular `ElementMaybeSignal`. Only this time, when you use
`String` or `&str` it will be interpreted as CSS selector with `query_selector_all`.

But you can also use it with containers.

```rust
// Array of Option<web_sys::Element>
use_resize_observer([window().body(), document().query_selector("div > p.some-class").unsrap()]);

// Vec of &str. All of them will be interpreted as CSS selectors with query_selector_all() and the
// results will be merged into one Vec.
use_resize_observer(vec!["div > p.some-class", "p.some-class"]);

// Slice of NodeRef
let node_ref1 = create_node_ref::<Div>();
let node_ref2 = create_node_ref::<Div>();
use_resize_observer(vec![node_ref1, node_ref2].as_slice());
```

## Usage in Options

Some functions have options that take `Element(s)MaybeSignal`.
They can be used in the same way.

```rust
use_mouse_with_options(
    UseMouseOptions::default().target("div > p.some-class")
);
```

See also ["Excluding Elements" in `on_click_outside`](elements/on_click_outside.md#excluding-elements).