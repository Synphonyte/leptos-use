use default_struct_builder::DefaultBuilder;

/// Options for [`use_raf_fn_with_options`](crate::use_raf_fn_with_options).
#[derive(DefaultBuilder)]
pub struct UseRafFnOptions {
    /// Start the requestAnimationFrame loop immediately on creation. Defaults to `true`.
    /// If false, the loop will only start when you call `resume()`.
    pub(crate) immediate: bool,
}

impl Default for UseRafFnOptions {
    fn default() -> Self {
        Self { immediate: true }
    }
}
