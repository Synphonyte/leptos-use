use default_struct_builder::DefaultBuilder;
use leptos::*;

///
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_element_bounding)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_element_bounding;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// use_element_bounding();
/// #
/// # view! { }
/// # }
/// ```
pub fn use_element_bounding() -> UseElementBoundingReturn {
    use_element_bounding_with_options(UseElementBoundingOptions::default())
}

/// Version of [`use_element_bounding`] that takes a `UseElementBoundingOptions`. See [`use_element_bounding`] for how to use.
pub fn use_element_bounding_with_options(options: UseElementBoundingOptions) -> UseElementBoundingReturn {
    UseElementBoundingReturn {}
}

/// Options for [`use_element_bounding_with_options`].
#[derive(DefaultBuilder)]
pub struct UseElementBoundingOptions {}

impl Default for UseElementBoundingOptions {
    fn default() -> Self {
        Self {}
    }
}

/// Return type of [`use_element_bounding`].
pub struct UseElementBoundingReturn {}