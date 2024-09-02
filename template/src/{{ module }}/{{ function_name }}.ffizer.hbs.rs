use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;

///{{#if (eq unstable_apis "true")}}
///
/// > This function requires `--cfg=web_sys_unstable_apis` to be activated as
/// > [described in the wasm-bindgen guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html).{{/if}}
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/{{ function_name }})
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use{{#if module}}::{{ module }}{{/if}}::{{ function_name }};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// {{ function_name }}();
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
// #[doc(cfg(feature = "{{feature}}"))]
pub fn {{ function_name }}() -> {{ to_pascal_case function_name }}Return {
    {{ function_name }}_with_options({{ to_pascal_case function_name }}Options::default())
}

/// Version of [`fn@crate::{{ function_name }}`] that takes a `{{ to_pascal_case function_name }}Options`. See [`fn@crate::{{ function_name }}`] for how to use.{{#if feature}}
// #[doc(cfg(feature = "{{feature}}"))]{{/if}}
pub fn {{ function_name }}_with_options(options: {{ to_pascal_case function_name }}Options) -> {{ to_pascal_case function_name }}Return {
    {{ to_pascal_case function_name }}Return {}
}

/// Options for [`fn@crate::{{ function_name }}_with_options`].{{#if feature}}
// #[doc(cfg(feature = "{{feature}}"))]{{/if}}
#[derive(DefaultBuilder)]
pub struct {{ to_pascal_case function_name }}Options {}

impl Default for {{ to_pascal_case function_name }}Options {
    fn default() -> Self {
        Self {}
    }
}

/// Return type of [`fn@crate::{{ function_name }}`].{{#if feature}}
// #[doc(cfg(feature = "{{feature}}"))]{{/if}}
pub struct {{ to_pascal_case function_name }}Return {}