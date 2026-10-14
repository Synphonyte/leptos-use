use default_struct_builder::DefaultBuilder;

/// Options for [`fn@crate::{{ function_name }}_with_options`].
#[derive(DefaultBuilder)]
pub struct {{ to_pascal_case function_name }}Options {}

impl Default for {{ to_pascal_case function_name }}Options {
    fn default() -> Self {
        Self {}
    }
}
