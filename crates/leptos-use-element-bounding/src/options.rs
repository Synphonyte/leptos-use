use default_struct_builder::DefaultBuilder;

/// Options for [`use_element_bounding_with_options`](crate::use_element_bounding_with_options).
#[derive(DefaultBuilder)]
pub struct UseElementBoundingOptions {
    /// Reset values to 0 on component disposal
    ///
    /// Default: `true`
    pub reset: bool,

    /// Listen to window resize event
    ///
    /// Default: `true`
    pub window_resize: bool,

    /// Listen to window scroll event
    ///
    /// Default: `true`
    pub window_scroll: bool,

    /// Immediately call update
    ///
    /// Default: `true`
    pub immediate: bool,
}

impl Default for UseElementBoundingOptions {
    fn default() -> Self {
        Self {
            reset: true,
            window_resize: true,
            window_scroll: true,
            immediate: true,
        }
    }
}
