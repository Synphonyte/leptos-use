use default_struct_builder::DefaultBuilder;

/// Options for [`use_interval_fn_with_options`](crate::use_interval_fn_with_options)
#[derive(DefaultBuilder)]
pub struct UseIntervalFnOptions {
    /// Start the timer immediately. Defaults to `true`.
    pub immediate: bool,

    /// Execute the callback immediate after calling this function. Defaults to `false`
    pub immediate_callback: bool,
}

impl Default for UseIntervalFnOptions {
    fn default() -> Self {
        Self {
            immediate: true,
            immediate_callback: false,
        }
    }
}
