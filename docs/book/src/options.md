# Options

Most functions in Leptos-Use come with a version `..._with_options`. For example `use_css_var` has a
version `use_css_var_with_options`. As the name suggests, you can provide additional options to those versions of the
functions.

These options are defined as structs with the corresponding PascalCase name. For our example `use_css_var_with_options`
the name of the struct is `UseCssVarOptions`. Every option struct implements `Default` and the builder pattern to
make it easy to change only the values needed. This can look like the following example.

```rust
let (color, set_color) = use_css_var_with_options(
    "--color",
    UseCssVarOptions::default()
        .target(el)
        .initial_value("#eee"),
);
```

Here only the values `target` and `initial_value` are changed and everything else is left to default.

TODO : automatic conversion like Fn and Option