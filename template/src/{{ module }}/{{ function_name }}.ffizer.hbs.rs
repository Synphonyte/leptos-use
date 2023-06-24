use default_struct_builder::DefaultBuilder;
use leptos::*;

///
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/{{ function_name }})
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// use leptos_use{{#if module}}::{{ module }}{{/if}}::{{ function_name }};
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// {{ function_name }}({{#if scope }}cx{{/if}});
/// #
/// # view! { cx, }
/// # }
/// ```{{#if feature}}
// #[doc(cfg(feature = "{{feature}}"))]{{/if}}
pub fn {{ function_name }}({{#if scope }}cx: Scope{{/if}}) -> {{ to_pascal_case function_name }}Return {
    {{ function_name }}_with_options({{#if scope }}cx, {{/if}}{{ to_pascal_case function_name }}Options::default())
}

/// Version of [`{{ function_name }}`] that takes a `{{ to_pascal_case function_name }}Options`. See [`{{ function_name }}`] for how to use.{{#if feature}}
// #[doc(cfg(feature = "{{feature}}"))]{{/if}}
pub fn {{ function_name }}_with_options({{#if scope }}cx: Scope, {{/if}}options: {{ to_pascal_case function_name }}Options) -> {{ to_pascal_case function_name }}Return {
    {{ to_pascal_case function_name }}Return {}
}

/// Options for [`{{ function_name }}_with_options`].{{#if feature}}
// #[doc(cfg(feature = "{{feature}}"))]{{/if}}
#[derive(DefaultBuilder)]
pub struct {{ to_pascal_case function_name }}Options {}

impl Default for {{ to_pascal_case function_name }}Options {
    fn default() -> Self {
        Self {}
    }
}

/// Return type of [`{{ function_name }}`].{{#if feature}}
// #[doc(cfg(feature = "{{feature}}"))]{{/if}}
pub struct {{ to_pascal_case function_name }}Return {}